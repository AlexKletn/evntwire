use serde::Deserialize;

use crate::{InputGesture, PhysicalButton, PhysicalSwitch};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    #[serde(default, rename = "gesture")]
    pub gestures: Vec<InputGesture>,
    #[serde(default, rename = "button")]
    pub buttons: Vec<PhysicalButton>,
    #[serde(default, rename = "switch")]
    pub switches: Vec<PhysicalSwitch>,
}
