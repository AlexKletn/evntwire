use std::time::{Duration, Instant};

use evntwire_config::RepeatPolicy;

use super::repeat_mode::RepeatMode;

pub struct RepeatTracker {
    mode: RepeatMode,
    next: Option<Instant>,
}

impl RepeatTracker {
    pub fn new(policy: &RepeatPolicy) -> Result<Self, &'static str> {
        let mode = match *policy {
            RepeatPolicy::Native => RepeatMode::Native,
            RepeatPolicy::Off => RepeatMode::Off,
            RepeatPolicy::Timer { delay_ms, rate_hz }
                if delay_ms > 0 && (1..=100).contains(&rate_hz) =>
            {
                RepeatMode::Timer {
                    delay: Duration::from_millis(u64::from(delay_ms)),
                    interval: Duration::from_secs(1) / rate_hz,
                }
            }
            RepeatPolicy::Timer { .. } => {
                return Err("Repeat timer requires a positive delay and a rate from 1 to 100 Hz");
            }
        };
        Ok(Self { mode, next: None })
    }

    pub fn accepts_native_repeat(&self) -> bool {
        matches!(self.mode, RepeatMode::Native)
    }

    pub fn arm(&mut self, pressed_at: Instant) {
        if let RepeatMode::Timer { delay, .. } = self.mode {
            self.next = pressed_at.checked_add(delay);
        }
    }

    pub fn cancel(&mut self) {
        self.next = None;
    }

    pub fn deadline(&self) -> Option<Instant> {
        self.next
    }

    pub fn advance(&mut self, now: Instant) -> bool {
        let RepeatMode::Timer { interval, .. } = self.mode else {
            return false;
        };
        if self.next.is_none_or(|deadline| now < deadline) {
            return false;
        }

        // A delayed event loop emits one repeat instead of catching up in a burst.
        self.next = now.checked_add(interval);
        true
    }
}

#[cfg(test)]
#[path = "repeat_tracker.tests.rs"]
mod tests;
