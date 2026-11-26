use std::collections::BTreeMap;
use std::io;

use evdev::{KeyCode, SwitchCode};
use evntwire_config::Config;

#[derive(Default)]
pub(super) struct Mappings<'a> {
    pub keys: BTreeMap<u16, (&'a str, &'a str)>,
    pub gestures: BTreeMap<i32, (&'a str, &'a str)>,
    pub switches: BTreeMap<u16, &'a str>,
}

impl<'a> Mappings<'a> {
    pub fn from_config(config: &'a Config) -> io::Result<BTreeMap<&'a str, Self>> {
        let mut devices = BTreeMap::<&str, Self>::new();
        let extra_buttons = config.inputs.values().flat_map(|input| &input.buttons);
        for binding in config.physical_buttons.iter().chain(extra_buttons) {
            let code: KeyCode = binding
                .key
                .parse()
                .map_err(|_| invalid(format!("Unknown evdev key {:?}", binding.key)))?;
            let source = binding.source.as_deref().unwrap_or(&binding.device_name);
            let keys = &mut devices.entry(&binding.device_name).or_default().keys;
            if keys.insert(code.0, (source, &binding.button)).is_some() {
                return Err(invalid(format!(
                    "Duplicate evdev key {:?} on {:?}",
                    binding.key, binding.device_name
                )));
            }
        }
        let extra_switches = config.inputs.values().flat_map(|input| &input.switches);
        for binding in config.physical_switches.iter().chain(extra_switches) {
            let code: SwitchCode = binding
                .switch
                .parse()
                .map_err(|_| invalid(format!("Unknown evdev switch {:?}", binding.switch)))?;
            let switches = &mut devices.entry(&binding.device_name).or_default().switches;
            if switches.insert(code.0, &binding.state).is_some() {
                return Err(invalid(format!(
                    "Duplicate evdev switch {:?} on {:?}",
                    binding.switch, binding.device_name
                )));
            }
        }
        for binding in config.inputs.values().flat_map(|input| &input.gestures) {
            let source = binding.source.as_deref().unwrap_or(&binding.device_name);
            let gestures = &mut devices.entry(&binding.device_name).or_default().gestures;
            if gestures
                .insert(binding.value, (source, &binding.button))
                .is_some()
            {
                return Err(invalid(format!(
                    "Duplicate MSC_GESTURE value {} on {:?}",
                    binding.value, binding.device_name
                )));
            }
        }
        Ok(devices)
    }
}

fn invalid(message: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}
