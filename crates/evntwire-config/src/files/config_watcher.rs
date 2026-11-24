use std::path::Path;

use notify::{Error, Event, RecommendedWatcher, RecursiveMode, Watcher};

pub struct ConfigWatcher {
    _watcher: RecommendedWatcher,
}

impl ConfigWatcher {
    pub fn watch(
        path: impl AsRef<Path>,
        mut on_change: impl FnMut(notify::Result<()>) + Send + 'static,
    ) -> notify::Result<Self> {
        let path = path.as_ref();
        let filename = path
            .file_name()
            .ok_or_else(|| Error::generic("Expected a configuration file path"))?;
        let directory = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."))
            .canonicalize()
            .map_err(Error::io)?;
        let config_path = directory.join(filename);
        let mut watcher =
            notify::recommended_watcher(move |result: notify::Result<Event>| match result {
                Ok(event) => {
                    let changes_file =
                        event.kind.is_create() || event.kind.is_modify() || event.kind.is_remove();
                    if event.need_rescan() || (changes_file && event.paths.contains(&config_path)) {
                        on_change(Ok(()));
                    }
                }
                Err(error) => on_change(Err(error)),
            })?;

        // Atomic saves replace the file, so keep the watch on its parent directory.
        watcher.watch(&directory, RecursiveMode::NonRecursive)?;
        Ok(Self { _watcher: watcher })
    }
}
