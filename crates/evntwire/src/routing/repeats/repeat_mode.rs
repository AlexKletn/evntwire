use std::time::Duration;

pub(super) enum RepeatMode {
    Native,
    Off,
    Timer { delay: Duration, interval: Duration },
}
