use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Action {
    Key {
        code: String,
    },
    Command {
        program: String,
        #[serde(default)]
        args: Vec<String>,
    },
}
