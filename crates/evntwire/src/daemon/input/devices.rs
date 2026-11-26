use std::collections::BTreeMap;
use std::io;
use std::os::fd::AsFd;
use std::path::PathBuf;
use std::time::Instant;

use evdev::{Device, EventType, MiscCode, SwitchCode};
use evntwire::{Event, Router};
use evntwire_config::{Action, Config};
use nix::poll::{PollFd, PollFlags};

use super::mappings::Mappings;

pub(in crate::daemon) struct InputDevices<'a> {
    mappings: BTreeMap<&'a str, Mappings<'a>>,
    devices: BTreeMap<PathBuf, Device>,
}

impl<'a> InputDevices<'a> {
    pub fn new(config: &'a Config) -> io::Result<Self> {
        Ok(Self {
            mappings: Mappings::from_config(config)?,
            devices: BTreeMap::new(),
        })
    }

    pub fn discover(&mut self, router: &mut Router<'a>) {
        if self.mappings.is_empty() {
            return;
        }
        for (path, device) in evdev::enumerate() {
            if self.devices.contains_key(&path) {
                continue;
            }
            let Some(name) = device.name() else { continue };
            let Some(mappings) = self.mappings.get(name) else {
                continue;
            };
            if self
                .devices
                .values()
                .any(|opened| opened.name() == Some(name))
            {
                continue;
            }
            if let Err(error) = device.set_nonblocking(true) {
                eprintln!("evntwire: {}: {error}", path.display());
                continue;
            }
            if let Some(supported) = device.supported_switches()
                && !mappings.switches.is_empty()
            {
                match device.get_switch_state() {
                    Ok(state) => {
                        for (&code, &name) in &mappings.switches {
                            let code = SwitchCode(code);
                            if supported.contains(code) {
                                router.set_state(name, Some(state.contains(code)));
                            }
                        }
                    }
                    Err(error) => eprintln!("evntwire: {}: {error}", path.display()),
                }
            }
            self.devices.insert(path, device);
        }
    }

    pub fn poll_descriptors(&self) -> impl Iterator<Item = PollFd<'_>> {
        self.devices
            .values()
            .map(|device| PollFd::new(device.as_fd(), PollFlags::POLLIN))
    }

    pub fn receive(
        &mut self,
        readiness: Vec<PollFlags>,
        router: &mut Router<'a>,
    ) -> Vec<&'a Action> {
        let mut actions = Vec::new();
        let mut disconnected = Vec::new();
        for ((path, device), flags) in self.devices.iter_mut().zip(readiness) {
            let Some(mappings) = device.name().and_then(|name| self.mappings.get(name)) else {
                continue;
            };
            if flags.intersects(PollFlags::POLLERR | PollFlags::POLLHUP | PollFlags::POLLNVAL) {
                disconnected.push(path.clone());
                continue;
            }
            if !flags.contains(PollFlags::POLLIN) {
                continue;
            }
            match device.fetch_events() {
                Ok(events) => {
                    for input in events {
                        match input.event_type() {
                            EventType::KEY => {
                                let Some(&(source, button)) = mappings.keys.get(&input.code())
                                else {
                                    continue;
                                };
                                let kind = match input.value() {
                                    0 => "release",
                                    1 => "press",
                                    2 => "repeat",
                                    _ => continue,
                                };
                                actions.extend(router.dispatch(
                                    &Event {
                                        source,
                                        button,
                                        kind,
                                    },
                                    Instant::now(),
                                ));
                            }
                            EventType::SWITCH => {
                                if let (Some(&name), value @ (0 | 1)) =
                                    (mappings.switches.get(&input.code()), input.value())
                                {
                                    router.set_state(name, Some(value == 1));
                                }
                            }
                            EventType::MISC if input.code() == MiscCode::MSC_GESTURE.0 => {
                                if let Some(&(source, button)) =
                                    mappings.gestures.get(&input.value())
                                {
                                    actions.extend(router.dispatch(
                                        &Event {
                                            source,
                                            button,
                                            kind: "gesture",
                                        },
                                        Instant::now(),
                                    ));
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
                Err(error) => {
                    eprintln!("evntwire: {}: {error}", path.display());
                    disconnected.push(path.clone());
                }
            }
        }

        for path in disconnected {
            if let Some(device) = self.devices.remove(&path)
                && let Some(mappings) = device.name().and_then(|name| self.mappings.get(name))
            {
                for &(source, button) in mappings.keys.values() {
                    router.cancel_button(source, button);
                }
                for name in mappings.switches.values() {
                    router.set_state(name, None);
                }
            }
        }
        actions
    }
}
