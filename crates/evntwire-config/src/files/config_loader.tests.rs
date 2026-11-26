use super::{Table, merge_tables};

#[test]
fn merges_nested_tables_and_replaces_arrays() {
    let mut base: Table = "[source]\nkeep = true\nkeys = [1, 2]".parse().unwrap();
    let overlay = "[source]\nkeys = [3]".parse().unwrap();

    merge_tables(&mut base, overlay);

    let expected: Table = "[source]\nkeep = true\nkeys = [3]".parse().unwrap();
    assert_eq!(base, expected);
}

#[test]
fn preserves_base_buttons_when_loading_a_named_input_group() {
    let mut base: Table = r#"
        [[physical_button]]
        device_name = "Power buttons"
        key = "KEY_POWER"
        button = "power"
    "#
    .parse()
    .unwrap();
    let overlay = r#"
        [[input.sensors.button]]
        device_name = "Sensor gestures"
        key = "KEY_F24"
        button = "tilt"

        [[input.sensors.switch]]
        device_name = "Sensor lid"
        switch = "SW_LID"
        state = "lid.closed"
    "#
    .parse()
    .unwrap();

    merge_tables(&mut base, overlay);

    let config: crate::Config = base.try_into().unwrap();
    assert_eq!(config.physical_buttons[0].button, "power");
    assert_eq!(config.inputs["sensors"].buttons[0].button, "tilt");
    assert_eq!(config.inputs["sensors"].switches[0].state, "lid.closed");
}
