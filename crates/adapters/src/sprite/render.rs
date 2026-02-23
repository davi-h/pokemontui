use std::process::Command;

pub struct SpriteRenderer;

impl SpriteRenderer {
    pub fn show(name: &str) {
        Command::new("pokeget")
            .arg(name)
            .status()
            .ok();
    }
}