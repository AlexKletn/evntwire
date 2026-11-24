use std::{fs, io, path::Path};

use crate::ConfigLoadError;

pub fn ensure_config_file(path: &Path) -> Result<(), ConfigLoadError> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).map_err(|error| ConfigLoadError::new(parent, error))?;
    }

    match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
    {
        Ok(_) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Ok(()),
        Err(error) => Err(ConfigLoadError::new(path, error)),
    }
}
