use serde::Deserialize;

use crate::{Binding, Combination, Gesture, PhysicalButton, Repeat};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default, rename = "physical_button")]
    pub physical_buttons: Vec<PhysicalButton>,
    #[serde(default, rename = "binding")]
    pub bindings: Vec<Binding>,
    #[serde(default, rename = "combination")]
    pub combinations: Vec<Combination>,
    #[serde(default, rename = "repeat")]
    pub repeats: Vec<Repeat>,
    #[serde(default, rename = "gesture")]
    pub gestures: Vec<Gesture>,
}

#[cfg(test)]
#[path = "config.tests.rs"]
mod tests;
