use std::process::Command;

pub struct SpriteLoader;

impl SpriteLoader {
    pub fn fetch(name: &str) {
        let _ = Command::new("pokeget")
            .arg(name)
            .status();
    }
}