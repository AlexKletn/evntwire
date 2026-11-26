use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalButton {
    pub device_name: String,
    pub source: Option<String>,
    pub key: String,
    pub button: String,
}
