use std::time::{Duration, Instant};

use evntwire_config::Gesture;

use super::{GestureTrigger, gesture_state::GestureState};

pub struct GestureTracker {
    hold: Duration,
    double_window: Duration,
    state: GestureState,
}

impl GestureTracker {
    pub fn new(config: &Gesture) -> Self {
        Self {
            hold: Duration::from_millis(u64::from(config.long_ms)),
            double_window: Duration::from_millis(u64::from(config.double_ms)),
            state: GestureState::Idle,
        }
    }

    pub fn press(&mut self, now: Instant, combination_window_ms: u64) -> Option<GestureTrigger> {
        let trigger = self.advance(now);
        let hold = self.hold.max(Duration::from_millis(combination_window_ms));
        match self.state {
            GestureState::Idle => {
                self.state = GestureState::FirstPress {
                    deadline: now + hold,
                };
            }
            GestureState::AwaitingSecondPress { .. } => {
                self.state = GestureState::SecondPress {
                    deadline: now + hold,
                };
            }
            _ => {}
        }
        trigger
    }

    pub fn release(&mut self, now: Instant) -> Option<GestureTrigger> {
        if !matches!(
            self.state,
            GestureState::FirstPress { .. }
                | GestureState::SecondPress { .. }
                | GestureState::LongTriggered
        ) {
            return None;
        }

        let trigger = self.advance(now);
        match std::mem::replace(&mut self.state, GestureState::Idle) {
            GestureState::FirstPress { .. } => {
                self.state = GestureState::AwaitingSecondPress {
                    deadline: now + self.double_window,
                };
                None
            }
            GestureState::SecondPress { .. } => Some(GestureTrigger::Double),
            _ => trigger,
        }
    }

    pub fn advance(&mut self, now: Instant) -> Option<GestureTrigger> {
        if self.deadline().is_none_or(|deadline| now < deadline) {
            return None;
        }

        if matches!(self.state, GestureState::AwaitingSecondPress { .. }) {
            self.state = GestureState::Idle;
            return Some(GestureTrigger::Single);
        }

        self.state = GestureState::LongTriggered;
        Some(GestureTrigger::Long)
    }

    pub fn deadline(&self) -> Option<Instant> {
        match self.state {
            GestureState::FirstPress { deadline }
            | GestureState::AwaitingSecondPress { deadline }
            | GestureState::SecondPress { deadline } => Some(deadline),
            GestureState::Idle | GestureState::LongTriggered => None,
        }
    }

    pub fn cancel(&mut self) {
        self.state = GestureState::Idle;
    }
}

#[cfg(test)]
#[path = "gesture_tracker.tests.rs"]
mod tests;
