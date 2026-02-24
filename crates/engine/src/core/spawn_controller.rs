// engine/src/core/spawn_controller.rs

use crate::spawn::spawn_service::SpawnService;
use crate::render::pokeget_render::PokegetRenderer;

use crate::factory::error::FactoryError;

use contracts::rng::Rng;
use crate::factory::pokemon_factory::PokemonFactory;
use crate::spawn::environment::EnvironmentProvider;

use domain::pokemon::entity::Pokemon;

/// Controller central responsável por orquestrar:
/// spawn → geração → renderização
///
/// Ele não contém lógica de regra.
/// Apenas coordena os serviços já existentes.
pub struct SpawnController<F, R, E>
where
    F: PokemonFactory,
    R: Rng,
    E: EnvironmentProvider,
{
    spawner: SpawnService<F, R, E>,
    renderer: PokegetRenderer,
}

impl<F, R, E> SpawnController<F, R, E>
where
    F: PokemonFactory,
    R: Rng,
    E: EnvironmentProvider,
{
    /// Cria novo controller
    pub fn new(spawner: SpawnService<F, R, E>, renderer: PokegetRenderer) -> Self {
        Self { spawner, renderer }
    }

    /// Executa um ciclo completo de spawn
    pub fn spawn_once(&mut self, level: u8) -> Result<Pokemon, FactoryError> {
        let pokemon = self.spawner.spawn(level)?;

        // Renderiza sprite no terminal
        if let Err(err) = self.renderer.render(&pokemon) {
            eprintln!("Erro ao renderizar sprite: {}", err);
        }

        Ok(pokemon)
    }

    /// Permite trocar RNG em runtime
    pub fn set_rng(&mut self, rng: R) {
        self.spawner.set_rng(rng);
    }

    /// Retorna referência ao spawner (caso precise acessar infos externas)
    pub fn spawner(&self) -> &SpawnService<F, R, E> {
        &self.spawner
    }

    /// Retorna referência mutável ao spawner
    pub fn spawner_mut(&mut self) -> &mut SpawnService<F, R, E> {
        &mut self.spawner
    }
}