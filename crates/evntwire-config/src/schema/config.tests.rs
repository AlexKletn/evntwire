use super::Config;
use crate::{Action, RepeatPolicy};

#[test]
fn reads_button_manager_sections_and_physical_mappings() {
    let config: Config = toml::from_str(
        r#"
        [[physical_button]]
        device_name = "gpio-keys"
        key = "KEY_VOLUMEUP"
        button = "volume-up"

        [[binding]]
        source = "stylus"
        button = "side"
        event = "press"
        when = { "stylus.in-range" = true }
        action = { type = "command", program = "/usr/bin/logger" }

        [[combination]]
        buttons = [{ button = "volume-up" }, { button = "volume-down" }]
        action = { type = "key", code = "KEY_MUTE" }

        [[repeat]]
        button = "volume-up"
        policy = { type = "timer", delay_ms = 300, rate_hz = 5 }

        [[gesture]]
        button = "power"
        long_ms = 800
        double_ms = 250
        long_action = { type = "key", code = "KEY_SLEEP" }
        double_action = { type = "key", code = "KEY_WAKEUP" }
    "#,
    )
    .unwrap();

    assert_eq!(config.physical_buttons[0].key, "KEY_VOLUMEUP");
    assert!(config.bindings[0].when["stylus.in-range"]);
    assert!(matches!(&config.bindings[0].action, Action::Command { args, .. } if args.is_empty()));
    assert_eq!(config.combinations[0].window_ms, 120);
    assert!(config.combinations[0].buttons[0].source.is_empty());
    assert!(matches!(
        config.repeats[0].policy,
        RepeatPolicy::Timer {
            delay_ms: 300,
            rate_hz: 5
        }
    ));
    assert_eq!(config.gestures[0].long_ms, 800);
}

#[test]
fn rejects_unknown_fields_in_a_binding() {
    let result = toml::from_str::<Config>(
        r#"
        [[binding]]
        button = "power"
        event = "press"
        action = { type = "key", code = "KEY_POWER" }
        unexpected = true
    "#,
    );

    assert!(result.is_err());
}
