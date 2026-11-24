use std::path::{Path, PathBuf};

pub struct ConfigPaths {
    file: PathBuf,
}

impl ConfigPaths {
    pub fn new(file: Option<PathBuf>) -> Self {
        Self {
            file: file.unwrap_or_else(|| PathBuf::from("/etc/evntwire/evntwire.toml")),
        }
    }

    pub fn file(&self) -> &Path {
        &self.file
    }

    pub fn drop_in_dir(&self) -> PathBuf {
        if self
            .file
            .extension()
            .is_some_and(|extension| extension == "toml")
        {
            return self.file.with_extension("d");
        }

        let mut directory = self.file.as_os_str().to_os_string();
        directory.push(".d");
        PathBuf::from(directory)
    }
}
