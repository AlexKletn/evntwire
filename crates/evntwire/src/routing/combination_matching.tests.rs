use evntwire_config::{Action, Button};

use super::{
    BTreeMap, Combination, Duration, Instant, combination_window_ms, matching_combination,
};

#[test]
fn longest_match_wins_and_equal_length_keeps_configuration_order() {
    let combinations = [
        combination(&["a", "b"], 120),
        combination(&["a", "b", "c"], 160),
        combination(&["a", "b", "c"], 200),
    ];
    let start = Instant::now();
    let now = start + Duration::from_millis(120);
    let held = BTreeMap::from([(("", "a"), start), (("", "b"), now), (("", "c"), now)]);

    let selected = matching_combination(&combinations, ("", "b"), &held, &BTreeMap::new(), now);

    assert_eq!(selected.unwrap().window_ms, 160);
    assert_eq!(combination_window_ms(&combinations, ("", "b")), 200);
}

#[test]
fn checks_held_buttons_window_and_conditions() {
    let mut candidate = combination(&["a", "b"], 120);
    candidate.when.insert("enabled".into(), true);
    let combinations = [candidate];
    let start = Instant::now();
    let now = start + Duration::from_millis(120);
    let mut held = BTreeMap::from([(("", "a"), start), (("", "b"), now)]);
    let states = BTreeMap::from([("enabled".into(), true)]);
    let select = |held: &BTreeMap<_, _>, states: &BTreeMap<_, _>, now| {
        matching_combination(&combinations, ("", "b"), held, states, now)
    };

    assert!(select(&held, &states, now).is_some());
    assert!(select(&held, &states, now + Duration::from_millis(1)).is_none());
    assert!(select(&held, &BTreeMap::new(), now).is_none());
    held.remove(&("", "a"));
    assert!(select(&held, &states, now).is_none());
}

fn combination(buttons: &[&str], window_ms: u64) -> Combination {
    Combination {
        when: BTreeMap::new(),
        buttons: buttons
            .iter()
            .map(|button| Button {
                source: String::new(),
                button: (*button).into(),
            })
            .collect(),
        window_ms,
        action: Action::Key {
            code: "KEY_MUTE".into(),
        },
    }
}
