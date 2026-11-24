use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum RepeatPolicy {
    Native,
    Off,
    Timer { delay_ms: u32, rate_hz: u32 },
}
