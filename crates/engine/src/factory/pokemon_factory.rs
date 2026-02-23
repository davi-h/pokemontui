use contracts::rng::Rng;

/// Trait for creating Pokemon. Move to contracts if needed by multiple crates.
pub trait PokemonFactory {
    fn create(&self, name: &str, level: u8) -> domain::pokemon::entity::Pokemon;
}
use domain::pokemon::{builder::PokemonBuilder, stats::Stats};

pub struct DefaultPokemonFactory<R: Rng> {
    rng: R,
}

impl<R: Rng> DefaultPokemonFactory<R> {
    pub fn new(rng: R) -> Self {
        Self { rng }
    }
}

impl<R: Rng> PokemonFactory for DefaultPokemonFactory<R> {
    fn create(&self, name: &str, level: u8) -> domain::pokemon::entity::Pokemon {
        let stats = Stats {
            hp: self.rng.u32(20, 40) as u16,
            attack: self.rng.u32(10, 30) as u16,
            defense: self.rng.u32(10, 30) as u16,
            speed: self.rng.u32(10, 30) as u16,
        };

        PokemonBuilder::new(name.into())
            .level(level)
            .stats(stats)
            .build()
    }
}