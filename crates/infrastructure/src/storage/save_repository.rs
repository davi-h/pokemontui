use std::{fs, path::PathBuf};

pub struct SaveRepository {
    path: PathBuf,
}

impl SaveRepository {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn save(&self, data: &str) {
        let _ = fs::write(&self.path, data);
    }

    pub fn load(&self) -> Option<String> {
        fs::read_to_string(&self.path).ok()
    }
}