use std::collections::BTreeMap;

use serde::Deserialize;

use crate::Action;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Binding {
    #[serde(default)]
    pub when: BTreeMap<String, bool>,
    #[serde(default)]
    pub source: String,
    pub button: String,
    pub event: String,
    pub action: Action,
}
