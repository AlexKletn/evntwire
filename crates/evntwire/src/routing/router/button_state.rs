use std::time::Instant;

use evntwire_config::Gesture;

use crate::{GestureTracker, RepeatTracker};

#[derive(Default)]
pub(super) struct ButtonState<'a> {
    pub(super) held_since: Option<Instant>,
    pub(super) pending: Option<Instant>,
    pub(super) consumed: bool,
    pub(super) gesture: Option<(&'a Gesture, GestureTracker)>,
    pub(super) repeat: Option<RepeatTracker>,
}

impl ButtonState<'_> {
    pub(super) fn deadline(&self) -> Option<Instant> {
        [
            self.pending,
            self.gesture
                .as_ref()
                .and_then(|(_, tracker)| tracker.deadline()),
            self.repeat.as_ref().and_then(RepeatTracker::deadline),
        ]
        .into_iter()
        .flatten()
        .min()
    }

    pub(super) fn consume(&mut self) {
        self.consumed = true;
        self.pending = None;
        if let Some((_, tracker)) = &mut self.gesture {
            tracker.cancel();
        }
        if let Some(repeat) = &mut self.repeat {
            repeat.cancel();
        }
    }
}
