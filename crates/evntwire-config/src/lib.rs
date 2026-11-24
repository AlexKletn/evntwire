mod files;
mod schema;

pub use files::{ConfigLoadError, ConfigPaths, ConfigWatcher, ensure_config_file, load_config};
pub use schema::{
    Action, Binding, Button, Combination, Config, Gesture, PhysicalButton, Repeat, RepeatPolicy,
};
