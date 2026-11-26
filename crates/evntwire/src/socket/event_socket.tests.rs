use std::{
    env,
    os::unix::fs::DirBuilderExt,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use evntwire_config::{Binding, Config};

use super::{Action, EventSocket, Instant, Router, UnixDatagram, fs, io};

#[test]
fn accepts_registered_events_round_trips_state_and_removes_socket_on_drop() {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let directory = env::temp_dir().join(format!("ew-{}-{stamp}", std::process::id()));
    fs::DirBuilder::new()
        .mode(0o700)
        .create(&directory)
        .unwrap();
    let server_path = directory.join("server");
    let client_path = directory.join("client");
    let mut server = EventSocket::bind(&server_path).unwrap();
    let client = UnixDatagram::bind(&client_path).unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    client.connect(&server_path).unwrap();
    let config = Config {
        bindings: vec![Binding {
            when: [("ready".into(), true)].into(),
            source: "stylus".into(),
            button: "side".into(),
            event: "press".into(),
            action: Action::Key {
                code: "KEY_MUTE".into(),
            },
        }],
        combinations: Vec::new(),
        gestures: Vec::new(),
        physical_buttons: Vec::new(),
        inputs: Default::default(),
        physical_switches: Vec::new(),
        repeats: Vec::new(),
    };
    let mut router = Router::new(&config).unwrap();
    let now = Instant::now();
    let mut request = |message: &[u8]| {
        client.send(message).unwrap();
        server.receive(&mut router, now)
    };

    assert_eq!(
        request(b"event\tstylus\tside\tpress").err().unwrap().kind(),
        io::ErrorKind::InvalidData
    );
    request(b"register\tstylus\tside").unwrap();
    request(b"state\tset\tready\ttrue").unwrap();
    request(b"state\tget\tready").unwrap();
    let mut response = [0; 16];
    let len = client.recv(&mut response).unwrap();
    assert_eq!(&response[..len], b"true");
    request(b"event\tstylus\tside\tpress").unwrap();
    let actions = request(b"event\tstylus\tside\trelease").unwrap();
    assert!(matches!(actions.as_slice(), [Action::Key { code }] if code == "KEY_MUTE"));
    assert_eq!(
        request(&[b'x'; 1025]).err().unwrap().kind(),
        io::ErrorKind::InvalidData
    );

    drop(server);
    assert!(!server_path.exists());
    fs::remove_file(client_path).unwrap();
    fs::remove_dir(directory).unwrap();
}
