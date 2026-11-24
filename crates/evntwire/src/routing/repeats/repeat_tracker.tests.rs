use super::{Duration, Instant, RepeatPolicy, RepeatTracker};

#[test]
fn only_native_policy_accepts_system_repeats() {
    let now = Instant::now();
    for (policy, accepts_native) in [(RepeatPolicy::Native, true), (RepeatPolicy::Off, false)] {
        let mut tracker = RepeatTracker::new(&policy).unwrap();
        tracker.arm(now);

        assert_eq!(tracker.accepts_native_repeat(), accepts_native);
        assert_eq!(tracker.deadline(), None);
        assert!(!tracker.advance(now));
    }
}

#[test]
fn timer_uses_delay_and_rate_skips_missed_ticks_and_can_be_cancelled() {
    let mut tracker = RepeatTracker::new(&RepeatPolicy::Timer {
        delay_ms: 300,
        rate_hz: 5,
    })
    .unwrap();
    let start = Instant::now();
    let first_due = start + Duration::from_millis(300);
    tracker.arm(start);
    assert!(!tracker.accepts_native_repeat());
    assert_eq!(tracker.deadline(), Some(first_due));
    assert!(!tracker.advance(start));
    assert!(tracker.advance(first_due));
    assert_eq!(
        tracker.deadline(),
        Some(first_due + Duration::from_millis(200))
    );

    let late = start + Duration::from_secs(2);
    assert!(tracker.advance(late));
    assert!(!tracker.advance(late));
    assert_eq!(tracker.deadline(), Some(late + Duration::from_millis(200)));
    tracker.cancel();
    assert_eq!(tracker.deadline(), None);
    assert!(!tracker.advance(late + Duration::from_secs(1)));
}

#[test]
fn rejects_invalid_timer_settings() {
    for (delay_ms, rate_hz) in [(0, 5), (300, 0), (300, 101)] {
        assert!(RepeatTracker::new(&RepeatPolicy::Timer { delay_ms, rate_hz }).is_err());
    }
}
