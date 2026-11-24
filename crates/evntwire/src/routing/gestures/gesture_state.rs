use std::time::Instant;

#[derive(Clone, Copy)]
pub(super) enum GestureState {
    Idle,
    FirstPress { deadline: Instant },
    AwaitingSecondPress { deadline: Instant },
    SecondPress { deadline: Instant },
    LongTriggered,
}
