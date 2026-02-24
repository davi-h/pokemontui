use std::{
    path::PathBuf,
    process::Command,
};

pub struct SpriteLoader {
    dir: PathBuf,
}

impl SpriteLoader {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    /// Caminho do sprite
    fn path(&self, name: &str) -> PathBuf {
        self.dir.join(format!("{name}.png"))
    }

    /// Verifica se sprite já existe
    pub fn exists(&self, name: &str) -> bool {
        self.path(name).exists()
    }

    /// Garante que sprite exista
    /// baixa se necessário
    pub fn ensure(&self, name: &str) {
        if self.exists(name) {
            return;
        }

        let _ = Command::new("pokeget")
            .arg(name)
            .arg("--output")
            .arg(self.path(name))
            .status();
    }

    /// Força download
    pub fn fetch(&self, name: &str) {
        let _ = Command::new("pokeget")
            .arg(name)
            .arg("--output")
            .arg(self.path(name))
            .status();
    }
}