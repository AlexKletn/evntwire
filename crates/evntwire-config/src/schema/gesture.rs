use serde::Deserialize;

use crate::Action;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gesture {
    #[serde(default)]
    pub source: String,
    pub button: String,
    pub long_ms: u32,
    pub double_ms: u32,
    pub long_action: Action,
    pub double_action: Action,
}
