use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputGesture {
    pub device_name: String,
    pub source: Option<String>,
    pub value: i32,
    pub button: String,
}
