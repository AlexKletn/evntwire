use std::collections::BTreeMap;

use serde::Deserialize;

use crate::{Action, Button};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Combination {
    #[serde(default)]
    pub when: BTreeMap<String, bool>,
    pub buttons: Vec<Button>,
    #[serde(default = "default_window_ms")]
    pub window_ms: u64,
    pub action: Action,
}

impl Combination {
    pub const DEFAULT_WINDOW_MS: u64 = 120;
}

fn default_window_ms() -> u64 {
    Combination::DEFAULT_WINDOW_MS
}
