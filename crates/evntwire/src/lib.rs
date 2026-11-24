mod execution;
mod routing;
#[cfg(unix)]
mod socket;

#[cfg(target_os = "linux")]
pub use execution::VirtualKeyboard;
pub use execution::{KeyOutput, execute_action};
pub use routing::{
    Event, GestureTracker, GestureTrigger, RepeatTracker, Router, combination_window_ms,
    matching_action, matching_combination,
};
#[cfg(unix)]
pub use socket::EventSocket;
