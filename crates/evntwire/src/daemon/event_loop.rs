use std::{
    error::Error,
    ffi::OsString,
    io,
    os::fd::AsFd,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use evntwire::{EventSocket, Router, VirtualKeyboard};
use evntwire_config::{Action, Config, ConfigPaths, ensure_config_file, load_config};
use nix::{
    errno::Errno,
    poll::{PollFd, PollFlags, poll},
};

use super::{command_pool::CommandPool, input::InputDevices};

const SOCKET_BATCH_SIZE: usize = 64;
const MAX_POLL_MS: u16 = 1000;

pub fn run(mut args: impl Iterator<Item = OsString>) -> Result<(), Box<dyn Error>> {
    let socket_path = args.next();
    let config_path = args.next().map(PathBuf::from);
    if socket_path.as_deref().is_some_and(|arg| arg == "--version") && config_path.is_none() {
        println!("evntwire {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let socket_path = socket_path
        .filter(|_| args.next().is_none())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Usage: evntwire SOCKET_PATH [CONFIG_PATH]",
            )
        })?;
    let config_paths = ConfigPaths::new(config_path);
    ensure_config_file(config_paths.file())?;
    let config = load_config::<Config>(&config_paths)?;
    let mut router = Router::new(&config)
        .map_err(|message| io::Error::new(io::ErrorKind::InvalidData, message))?;
    let mut socket = EventSocket::bind(Path::new(&socket_path))?;
    let mut keyboard = VirtualKeyboard::new(&config)?;
    let mut commands = CommandPool::default();
    let mut input = InputDevices::new(&config)?;
    let mut next_discovery = Instant::now();

    loop {
        commands.reap();
        let now = Instant::now();
        if now >= next_discovery {
            input.discover(&mut router);
            next_discovery = now + Duration::from_secs(1);
        }
        let deadline = router
            .deadline()
            .map_or(next_discovery, |time| time.min(next_discovery));
        let (socket_ready, input_ready) = wait_for_input(&socket, &input, deadline)?;
        execute(
            input.receive(input_ready, &mut router),
            &mut commands,
            &mut keyboard,
        );
        if socket_ready {
            // Bound the batch so a busy socket cannot starve routing timers.
            for _ in 0..SOCKET_BATCH_SIZE {
                match socket.receive(&mut router, Instant::now()) {
                    Ok(actions) => execute(actions, &mut commands, &mut keyboard),
                    Err(error) if error.kind() == io::ErrorKind::WouldBlock => break,
                    Err(error) => eprintln!("evntwire: socket: {error}"),
                }
            }
        }
        execute(router.advance(Instant::now()), &mut commands, &mut keyboard);
    }
}

fn wait_for_input(
    socket: &EventSocket,
    input: &InputDevices<'_>,
    deadline: Instant,
) -> io::Result<(bool, Vec<PollFlags>)> {
    let timeout_ms = deadline
        .saturating_duration_since(Instant::now())
        .as_nanos()
        .div_ceil(1_000_000)
        .min(u128::from(MAX_POLL_MS)) as u16;
    let mut descriptors = std::iter::once(PollFd::new(socket.as_fd(), PollFlags::POLLIN))
        .chain(input.poll_descriptors())
        .collect::<Vec<_>>();
    match poll(&mut descriptors, timeout_ms) {
        Ok(_) => {
            let socket_flags = descriptors[0].revents().unwrap_or_else(PollFlags::empty);
            if socket_flags
                .intersects(PollFlags::POLLERR | PollFlags::POLLHUP | PollFlags::POLLNVAL)
            {
                return Err(io::Error::other("Event socket poll failed"));
            }
            let input_ready = descriptors[1..]
                .iter()
                .map(|descriptor| descriptor.revents().unwrap_or_else(PollFlags::empty))
                .collect();
            Ok((socket_flags.contains(PollFlags::POLLIN), input_ready))
        }
        Err(Errno::EINTR) => Ok((false, Vec::new())),
        Err(error) => Err(error.into()),
    }
}

fn execute(actions: Vec<&Action>, commands: &mut CommandPool, keys: &mut VirtualKeyboard) {
    for action in actions {
        if let Err(error) = commands.execute(action, keys) {
            eprintln!("evntwire: action: {error}");
        }
    }
}
