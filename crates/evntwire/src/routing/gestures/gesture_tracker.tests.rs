use evntwire_config::Action;

use super::*;

#[test]
fn single_waits_for_the_double_press_window() {
    let start = Instant::now();
    let mut tracker = tracker();
    assert_eq!(tracker.press(start, 120), None);
    assert_eq!(tracker.release(start + Duration::from_millis(20)), None);
    assert_eq!(tracker.advance(start + Duration::from_millis(269)), None);
    assert_eq!(
        tracker.advance(start + Duration::from_millis(270)),
        Some(GestureTrigger::Single)
    );
    assert_eq!(tracker.deadline(), None);
}

#[test]
fn second_short_press_emits_double_on_release() {
    let start = Instant::now();
    let mut tracker = tracker();
    tracker.press(start, 120);
    tracker.release(start + Duration::from_millis(20));
    assert_eq!(tracker.press(start + Duration::from_millis(100), 120), None);
    assert_eq!(
        tracker.release(start + Duration::from_millis(120)),
        Some(GestureTrigger::Double)
    );
    assert_eq!(tracker.deadline(), None);
}

#[test]
fn long_press_emits_once_and_cancel_discards_a_pending_press() {
    let start = Instant::now();
    let mut tracker = tracker();
    let hold_deadline = start + Duration::from_millis(800);
    tracker.press(start, 120);
    assert_eq!(tracker.advance(hold_deadline), Some(GestureTrigger::Long));
    assert_eq!(tracker.release(hold_deadline), None);

    tracker.press(hold_deadline, 120);
    tracker.cancel();
    assert_eq!(tracker.deadline(), None);
    assert_eq!(
        tracker.advance(hold_deadline + Duration::from_secs(1)),
        None
    );
}

#[test]
fn a_longer_combination_window_postpones_the_long_press() {
    let start = Instant::now();
    let mut tracker = tracker();
    tracker.press(start, 1000);

    assert_eq!(tracker.advance(start + Duration::from_millis(800)), None);
    assert_eq!(
        tracker.advance(start + Duration::from_millis(1000)),
        Some(GestureTrigger::Long)
    );
}

fn tracker() -> GestureTracker {
    GestureTracker::new(&Gesture {
        source: String::new(),
        button: "power".into(),
        long_ms: 800,
        double_ms: 250,
        long_action: Action::Key {
            code: "KEY_SLEEP".into(),
        },
        double_action: Action::Key {
            code: "KEY_WAKEUP".into(),
        },
    })
}
