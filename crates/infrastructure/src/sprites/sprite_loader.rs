use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

pub struct SpriteLoader {
    dir: PathBuf,
}

impl SpriteLoader {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        let dir = dir.into();
        fs::create_dir_all(&dir).ok(); // garante diretório
        Self { dir }
    }

    fn path(&self, name: &str) -> PathBuf {
        self.dir.join(format!("{name}.png"))
    }

    pub fn exists(&self, name: &str) -> bool {
        self.path(name).exists()
    }

    /// Baixa sprite (core interno)
    fn download(&self, name: &str) -> Result<(), String> {
        let output = Command::new("pokeget")
            .arg(name)
            .arg("--output")
            .arg(self.path(name))
            .output()
            .map_err(|e| format!("failed to run pokeget: {e}"))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into());
        }

        Ok(())
    }

    /// garante sprite
    pub fn ensure(&self, name: &str) -> Result<(), String> {
        if self.exists(name) {
            return Ok(());
        }

        self.download(name)
    }

    /// força download
    pub fn fetch(&self, name: &str) -> Result<(), String> {
        self.download(name)
    }

    /// caminho público para render
    pub fn file(&self, name: &str) -> PathBuf {
        self.path(name)
    }
}