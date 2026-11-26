use evntwire_config::Binding;

use super::{Action, Config, KeyCode, VirtualKeyboard, configured_keys, io};

#[test]
fn collects_configured_linux_keys() {
    let config = config(Action::Key {
        code: "KEY_MUTE".into(),
    });
    let keys = configured_keys(&config).unwrap();

    assert!(keys.contains(KeyCode::KEY_MUTE));
    assert_eq!(keys.iter().count(), 1);
}

#[test]
fn rejects_invalid_key_before_opening_uinput() {
    let config = config(Action::Key {
        code: "MUTE_INVALID".into(),
    });
    let Err(error) = VirtualKeyboard::new(&config) else {
        panic!("invalid key was accepted");
    };
    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
}

#[test]
fn command_only_config_does_not_open_uinput() {
    let config = config(Action::Command {
        program: "/usr/bin/true".into(),
        args: Vec::new(),
    });
    assert!(VirtualKeyboard::new(&config).unwrap().device.is_none());
}

fn config(action: Action) -> Config {
    Config {
        bindings: vec![Binding {
            action,
            source: String::new(),
            button: "test".into(),
            event: "press".into(),
            when: Default::default(),
        }],
        physical_buttons: Vec::new(),
        inputs: Default::default(),
        physical_switches: Vec::new(),
        combinations: Vec::new(),
        gestures: Vec::new(),
        repeats: Vec::new(),
    }
}
