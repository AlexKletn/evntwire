use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Button {
    #[serde(default)]
    pub source: String,
    pub button: String,
}
