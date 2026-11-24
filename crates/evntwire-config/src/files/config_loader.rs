use std::{
    fs, io,
    path::{Path, PathBuf},
};

use serde::de::DeserializeOwned;
use toml::{Table, Value};

use crate::{ConfigLoadError, ConfigPaths};

/// Load the main file, then apply TOML fragments in filename order.
/// Tables merge recursively. Other values, including arrays, are replaced.
pub fn load_config<T: DeserializeOwned>(paths: &ConfigPaths) -> Result<T, ConfigLoadError> {
    let mut config = read_table(paths.file())?;
    for fragment in fragment_paths(&paths.drop_in_dir())? {
        merge_tables(&mut config, read_table(&fragment)?);
    }
    config
        .try_into()
        .map_err(|error| invalid_data(paths.file(), error))
}

fn read_table(path: &Path) -> Result<Table, ConfigLoadError> {
    let text = fs::read_to_string(path).map_err(|error| ConfigLoadError::new(path, error))?;
    text.parse().map_err(|error| invalid_data(path, error))
}

fn invalid_data(path: &Path, error: toml::de::Error) -> ConfigLoadError {
    ConfigLoadError::new(path, io::Error::new(io::ErrorKind::InvalidData, error))
}

fn fragment_paths(directory: &Path) -> Result<Vec<PathBuf>, ConfigLoadError> {
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(ConfigLoadError::new(directory, error)),
    };
    let mut fragments = Vec::new();
    for entry in entries {
        let path = entry
            .map_err(|error| ConfigLoadError::new(directory, error))?
            .path();
        if path
            .extension()
            .is_some_and(|extension| extension == "toml")
        {
            let metadata =
                fs::metadata(&path).map_err(|error| ConfigLoadError::new(&path, error))?;
            if metadata.is_file() {
                fragments.push(path);
            }
        }
    }
    fragments.sort_unstable();
    Ok(fragments)
}

fn merge_tables(base: &mut Table, overlay: Table) {
    for (key, value) in overlay {
        match (base.get_mut(&key), value) {
            (Some(Value::Table(base_table)), Value::Table(overlay_table)) => {
                merge_tables(base_table, overlay_table);
            }
            (_, value) => {
                base.insert(key, value);
            }
        }
    }
}

#[cfg(test)]
#[path = "config_loader.tests.rs"]
mod tests;
