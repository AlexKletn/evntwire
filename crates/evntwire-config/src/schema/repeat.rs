use serde::Deserialize;

use crate::RepeatPolicy;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Repeat {
    #[serde(default)]
    pub source: String,
    pub button: String,
    pub policy: RepeatPolicy,
}
