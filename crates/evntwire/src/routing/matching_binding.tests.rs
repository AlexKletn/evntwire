use super::*;

#[test]
fn selects_the_last_matching_action() {
    let bindings = [
        binding("stylus", "side", "press", "KEY_MUTE", BTreeMap::new()),
        binding(
            "stylus",
            "side",
            "press",
            "KEY_VOLUMEUP",
            BTreeMap::from([("in-range".into(), true)]),
        ),
        binding("other", "side", "press", "KEY_SLEEP", BTreeMap::new()),
        binding("stylus", "tip", "press", "KEY_SLEEP", BTreeMap::new()),
        binding("stylus", "side", "release", "KEY_SLEEP", BTreeMap::new()),
    ];
    let event = Event {
        source: "stylus",
        button: "side",
        kind: "press",
    };
    let states = BTreeMap::from([("in-range".into(), true)]);

    assert!(
        matches!(matching_action(&bindings, &event, &states), Some(Action::Key { code }) if code == "KEY_VOLUMEUP")
    );
    assert!(
        matches!(matching_action(&bindings, &event, &BTreeMap::new()), Some(Action::Key { code }) if code == "KEY_MUTE")
    );
}

#[test]
fn missing_state_does_not_match_an_explicit_false_condition() {
    let bindings = [binding(
        "stylus",
        "side",
        "press",
        "KEY_MUTE",
        BTreeMap::from([("in-range".into(), false)]),
    )];
    let event = Event {
        source: "stylus",
        button: "side",
        kind: "press",
    };

    assert!(matching_action(&bindings, &event, &BTreeMap::new()).is_none());
    assert!(
        matching_action(
            &bindings,
            &event,
            &BTreeMap::from([("in-range".into(), false)])
        )
        .is_some()
    );
}

fn binding(
    source: &str,
    button: &str,
    event: &str,
    code: &str,
    when: BTreeMap<String, bool>,
) -> Binding {
    Binding {
        source: source.into(),
        button: button.into(),
        event: event.into(),
        when,
        action: Action::Key { code: code.into() },
    }
}
