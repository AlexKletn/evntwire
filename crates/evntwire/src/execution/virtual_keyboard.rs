use std::io;

use evdev::{AttributeSet, KeyCode, KeyEvent, uinput::VirtualDevice};
use evntwire_config::{Action, Config};

use super::KeyOutput;

pub struct VirtualKeyboard {
    device: Option<VirtualDevice>,
    keys: AttributeSet<KeyCode>,
}

impl VirtualKeyboard {
    pub fn new(config: &Config) -> io::Result<Self> {
        let keys = configured_keys(config)?;
        let device = if keys.iter().next().is_some() {
            Some(
                VirtualDevice::builder()?
                    .name("evntwire")
                    .with_keys(&keys)?
                    .build()?,
            )
        } else {
            None
        };
        Ok(Self { device, keys })
    }
}

impl KeyOutput for VirtualKeyboard {
    fn tap(&mut self, code: &str) -> io::Result<()> {
        let key = parse_key(code)?;
        if !self.keys.contains(key) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Key {code} was not configured"),
            ));
        }
        let device = self
            .device
            .as_mut()
            .ok_or_else(|| io::Error::other("Virtual keyboard is unavailable"))?;
        device.emit(&[KeyEvent::new(key, 1).into()])?;
        device.emit(&[KeyEvent::new(key, 0).into()])
    }
}

fn configured_keys(config: &Config) -> io::Result<AttributeSet<KeyCode>> {
    let mut keys = AttributeSet::new();
    let actions = config
        .bindings
        .iter()
        .map(|binding| &binding.action)
        .chain(
            config
                .combinations
                .iter()
                .map(|combination| &combination.action),
        )
        .chain(
            config
                .gestures
                .iter()
                .flat_map(|gesture| [&gesture.long_action, &gesture.double_action]),
        );
    for action in actions {
        if let Action::Key { code } = action {
            keys.insert(parse_key(code)?);
        }
    }
    Ok(keys)
}

fn parse_key(code: &str) -> io::Result<KeyCode> {
    code.parse().map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid Linux key {code:?}: {error}"),
        )
    })
}

#[cfg(test)]
#[path = "virtual_keyboard.tests.rs"]
mod tests;
