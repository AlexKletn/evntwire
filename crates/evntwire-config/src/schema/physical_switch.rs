use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalSwitch {
    pub device_name: String,
    pub switch: String,
    pub state: String,
}
