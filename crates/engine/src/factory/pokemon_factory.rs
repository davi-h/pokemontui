use contracts::rng::Rng;
use domain::pokemon::{
    builder::PokemonBuilder,
    entity::Pokemon,
    stats::Stats,
};

use super::species_registry::SpeciesRegistry;
use super::error::FactoryError;

pub trait PokemonFactory {
    fn create(&self, name: &str, level: u8) -> Result<Pokemon, FactoryError>;
    fn create_random(&self, level: u8) -> Result<Pokemon, FactoryError>;
}

pub struct DefaultPokemonFactory<R, S>
where
    R: Rng,
    S: SpeciesRegistry,
{
    rng: R,
    registry: S,
}

impl<R, S> DefaultPokemonFactory<R, S>
where
    R: Rng,
    S: SpeciesRegistry,
{
    pub fn new(rng: R, registry: S) -> Self {
        Self { rng, registry }
    }

    fn roll_stats(&self) -> Stats {
        Stats {
            hp: self.rng.u32(20, 40) as u16,
            attack: self.rng.u32(10, 30) as u16,
            defense: self.rng.u32(10, 30) as u16,
            speed: self.rng.u32(10, 30) as u16,
        }
    }
}

impl<R, S> PokemonFactory for DefaultPokemonFactory<R, S>
where
    R: Rng,
    S: SpeciesRegistry,
{
    fn create(&self, name: &str, level: u8) -> Result<Pokemon, FactoryError> {
        let species = self
            .registry
            .get(name)
            .ok_or_else(|| FactoryError::UnknownSpecies(name.into()))?;

        Ok(
            PokemonBuilder::new(species.into())
                .level(level)
                .stats(self.roll_stats())
                .build()
        )
    }

    fn create_random(&self, level: u8) -> Result<Pokemon, FactoryError> {
        if self.registry.len() == 0 {
            return Err(FactoryError::EmptyRegistry);
        }

        let index = self.rng.range(0..self.registry.len());
        let species = self.registry.random(index).unwrap();

        self.create(species, level)
    }
}