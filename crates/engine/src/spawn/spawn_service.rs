use crate::factory::pokemon_factory::PokemonFactory;
use crate::factory::error::FactoryError;
use crate::spawn::rarity_engine::RarityEngine;
use crate::spawn::environment::{EnvironmentProvider, Environment};

use contracts::rng::Rng;
use domain::pokemon::entity::Pokemon;

/// Serviço responsável por gerar Pokémon baseado
/// em ambiente, raridade e RNG.
pub struct SpawnService<F, R, E>
where
    F: PokemonFactory,
    R: Rng,
    E: EnvironmentProvider,
{
    factory: F,
    rng: R,
    rarity: RarityEngine,
    env: E,
}

impl<F, R, E> SpawnService<F, R, E>
where
    F: PokemonFactory,
    R: Rng,
    E: EnvironmentProvider,
{
    pub fn new(factory: F, rng: R, rarity: RarityEngine, env: E) -> Self {
        Self {
            factory,
            rng,
            rarity,
            env,
        }
    }

    /// Gera um Pokémon com chance de shiny baseada no ambiente
    pub fn spawn(&mut self, level: u8) -> Result<Pokemon, FactoryError> {
        // factory precisa ser mutável caso implemente cache interno ou RNG interno
        let mut pokemon = self.factory.create_random(level)?;

        let environment = self.env.current();
        let chance = self.rarity.shiny_chance(&environment);

        if self.rng.float() < chance {
            pokemon.set_shiny(true);
        }

        Ok(pokemon)
    }

    /// Permite trocar RNG (útil pra testes determinísticos)
    pub fn set_rng(&mut self, rng: R) {
        self.rng = rng;
    }

    /// Retorna ambiente atual
    pub fn environment(&self) -> Environment {
        self.env.current()
    }

    /// Permite trocar provider de ambiente em runtime
    pub fn set_environment(&mut self, env: E) {
        self.env = env;
    }

    /// Acesso interno ao factory (ex: debug / metrics)
    pub fn factory(&self) -> &F {
        &self.factory
    }
}