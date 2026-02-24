use std::collections::HashSet;

use contracts::api::pokemon_data_source::PokemonDataSource;
use contracts::api::error::ApiError;
use contracts::rng::Rng;

use domain::pokemon::{
    builder::PokemonBuilder,
    stats::Stats,
    entity::Pokemon,
};

/// Trait responsável por construir Pokémons
pub trait PokemonFactory {
    fn create(&mut self, name: &str, level: u8) -> Result<Pokemon, FactoryError>;
    fn create_random(&mut self, level: u8) -> Result<Pokemon, FactoryError>;
}

#[derive(Debug)]
pub enum FactoryError {
    Api(ApiError),
    NoSpeciesAvailable,
    UnknownSpecies(String),
}

/// Implementação padrão de fábrica
pub struct DefaultPokemonFactory<R, D>
where
    R: Rng,
    D: PokemonDataSource,
{
    rng: R,
    data: D,

    /// pool para sorteio
    species_pool: Vec<String>,

    /// índice para validação rápida
    species_index: HashSet<String>,
}

impl<R, D> DefaultPokemonFactory<R, D>
where
    R: Rng,
    D: PokemonDataSource,
{
    pub fn new(rng: R, data: D, species_pool: Vec<String>) -> Self {
        let species_index = species_pool.iter().cloned().collect();

        Self {
            rng,
            data,
            species_pool,
            species_index,
        }
    }

    /// Cria Pokémon específico
    fn build(&mut self, name: &str, level: u8) -> Result<Pokemon, FactoryError> {
        let api_data = self.data.fetch(name).map_err(FactoryError::Api)?;

        let stats = Stats::from_base(
            api_data.base_stats.hp,
            api_data.base_stats.attack,
            api_data.base_stats.defense,
            api_data.base_stats.special_attack,
            api_data.base_stats.special_defense,
            api_data.base_stats.speed,
        )
        .scale_with_level(level);

        let shiny_roll = self.rng.range(0..4096);
        let shiny = shiny_roll == 0;

        Ok(
            PokemonBuilder::new(api_data.name)
                .level(level)
                .stats(stats)
                .shiny(shiny)
                .build()
        )
    }
}

impl<R, D> PokemonFactory for DefaultPokemonFactory<R, D>
where
    R: Rng,
    D: PokemonDataSource,
{
    /// Cria Pokémon específico
    fn create(&mut self, name: &str, level: u8) -> Result<Pokemon, FactoryError> {
        if !self.species_index.contains(name) {
            return Err(FactoryError::UnknownSpecies(name.into()));
        }

        self.build(name, level)
    }

    /// Cria Pokémon aleatório
    fn create_random(&mut self, level: u8) -> Result<Pokemon, FactoryError> {
        if self.species_pool.is_empty() {
            return Err(FactoryError::NoSpeciesAvailable);
        }

        let idx = self.rng.range(0..self.species_pool.len());

        // clone evita conflito de borrow mutável + imutável
        let name = self.species_pool[idx].clone();

        self.build(&name, level)
    }
}