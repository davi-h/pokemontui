use super::{entity::Pokemon, stats::Stats};

pub struct PokemonBuilder {
    name: String,
    level: u8,
    stats: Stats,
}

impl PokemonBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            level: 1,
            stats: Stats {
                hp: 1,
                attack: 1,
                defense: 1,
                speed: 1,
            },
        }
    }

    pub fn level(mut self, level: u8) -> Self {
        self.level = level;
        self
    }

    pub fn stats(mut self, stats: Stats) -> Self {
        self.stats = stats;
        self
    }

    pub fn build(self) -> Pokemon {
        Pokemon {
            name: self.name,
            level: self.level,
            stats: self.stats,
        }
    }
}