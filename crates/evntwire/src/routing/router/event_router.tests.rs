use evntwire_config::{Binding, Button, Gesture, Repeat};

use super::{
    Action, BTreeMap, Combination, Config, Duration, Event, Instant, RepeatPolicy, Router,
};

#[test]
fn combination_cancels_pending_single_actions_and_releases() {
    let mut config = config();
    config.combinations.push(Combination {
        buttons: ["a", "b"]
            .map(|button| Button {
                source: "test".into(),
                button: button.into(),
            })
            .into(),
        action: key("KEY_COMBO"),
        window_ms: 200,
        when: BTreeMap::new(),
    });
    let now = Instant::now();
    let mut router = Router::new(&config).unwrap();

    assert!(router.dispatch(&event("a", "press"), now).is_empty());
    assert_eq!(router.deadline(), Some(now + Duration::from_millis(200)));
    let later = now + Duration::from_millis(200);
    assert_key(&router.dispatch(&event("b", "press"), later), "KEY_COMBO");
    assert!(router.dispatch(&event("a", "release"), later).is_empty());
    assert!(router.dispatch(&event("b", "release"), later).is_empty());
    assert!(router.advance(later).is_empty());
}

#[test]
fn timer_repeat_routes_to_its_binding_and_stops_after_release() {
    let mut config = config();
    config.repeats.push(Repeat {
        source: "test".into(),
        button: "a".into(),
        policy: RepeatPolicy::Timer {
            delay_ms: 300,
            rate_hz: 5,
        },
    });
    let now = Instant::now();
    let repeat = now + Duration::from_millis(300);
    let mut router = Router::new(&config).unwrap();

    assert!(router.dispatch(&event("a", "press"), now).is_empty());
    assert_key(
        &router.advance(now + Duration::from_millis(120)),
        "KEY_PRESS",
    );
    assert_eq!(router.deadline(), Some(repeat));
    assert!(router.dispatch(&event("a", "repeat"), repeat).is_empty());
    assert_key(&router.advance(repeat), "KEY_REPEAT");
    assert_key(
        &router.dispatch(&event("a", "release"), repeat),
        "KEY_RELEASE",
    );
    assert_eq!(router.deadline(), None);
    assert!(router.advance(repeat + Duration::from_secs(1)).is_empty());
}

#[test]
fn long_gesture_routes_to_its_action_once() {
    let mut config = config();
    config.gestures.push(Gesture {
        source: "test".into(),
        button: "a".into(),
        long_ms: 400,
        double_ms: 250,
        long_action: key("KEY_LONG"),
        double_action: key("KEY_DOUBLE"),
    });
    let now = Instant::now();
    let deadline = now + Duration::from_millis(400);
    let mut router = Router::new(&config).unwrap();

    assert!(router.dispatch(&event("a", "press"), now).is_empty());
    assert_eq!(router.deadline(), Some(deadline));
    assert_key(&router.advance(deadline), "KEY_LONG");
    assert!(router.dispatch(&event("a", "release"), deadline).is_empty());
}

fn assert_key(actions: &[&Action], expected: &str) {
    assert!(matches!(actions, [Action::Key { code }] if code == expected));
}

fn event<'a>(button: &'a str, kind: &'a str) -> Event<'a> {
    Event {
        source: "test",
        button,
        kind,
    }
}

fn key(code: &str) -> Action {
    Action::Key { code: code.into() }
}

fn config() -> Config {
    Config {
        physical_buttons: Vec::new(),
        bindings: [
            ("press", "KEY_PRESS"),
            ("release", "KEY_RELEASE"),
            ("repeat", "KEY_REPEAT"),
        ]
        .into_iter()
        .map(|(kind, code)| Binding {
            source: "test".into(),
            button: "a".into(),
            event: kind.into(),
            when: BTreeMap::new(),
            action: key(code),
        })
        .collect(),
        combinations: Vec::new(),
        repeats: Vec::new(),
        gestures: Vec::new(),
    }
}
