use std::{fs, path::PathBuf};

pub struct FileCache {
    dir: PathBuf,
}

impl FileCache {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let path = self.dir.join(key);
        fs::read_to_string(path).ok()
    }

    pub fn set(&self, key: &str, value: &str) {
        let path = self.dir.join(key);
        let _ = fs::write(path, value);
    }
}