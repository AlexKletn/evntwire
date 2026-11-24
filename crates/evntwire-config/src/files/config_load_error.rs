use std::{
    error::Error,
    fmt, io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct ConfigLoadError {
    path: PathBuf,
    source: io::Error,
}

impl ConfigLoadError {
    pub(crate) fn new(path: &Path, source: io::Error) -> Self {
        Self {
            path: path.to_owned(),
            source,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl fmt::Display for ConfigLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.path.display(), self.source)
    }
}

impl Error for ConfigLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
