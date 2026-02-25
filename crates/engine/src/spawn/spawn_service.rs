use crate::factory::pokemon_factory::PokemonFactory;
use crate::spawn::{
    context::SpawnContext,
    engine::SpawnEngine,
    modifiers::modifier::SpawnModifier,
    table::SpawnTable,
    rarity_engine::RarityEngine,
};

use contracts::rng::Rng;
use domain::pokemon::entity::Pokemon;


/// Fonte de tabelas base.
///
/// Separado do EnvironmentProvider propositalmente
/// para não acoplar lógica de mundo com spawn tables.
pub trait SpawnTableProvider {
    fn table(&self, ctx: &SpawnContext) -> SpawnTable;
}


/// Serviço orquestrador do pipeline de spawn.
///
/// Pipeline:
/// Context -> BaseTable -> Modifiers -> Engine -> Rarity
pub struct SpawnService<F, R, P>
where
    F: PokemonFactory,
    R: Rng,
    P: SpawnTableProvider,
{
    engine: SpawnEngine<F>,
    rng: R,
    rarity: RarityEngine,
    table_provider: P,
    modifiers: Vec<Box<dyn SpawnModifier>>,
}

impl<F, R, P> SpawnService<F, R, P>
where
    F: PokemonFactory,
    R: Rng,
    P: SpawnTableProvider,
{
    pub fn new(
        factory: F,
        rng: R,
        rarity: RarityEngine,
        table_provider: P,
    ) -> Self {
        Self {
            engine: SpawnEngine::new(factory),
            rng,
            rarity,
            table_provider,
            modifiers: Vec::new(),
        }
    }

    /// Adiciona modifier ao pipeline
    pub fn add_modifier<M>(&mut self, modifier: M)
    where
        M: SpawnModifier + 'static,
    {
        self.modifiers.push(Box::new(modifier));

        // ordena automaticamente
        self.modifiers.sort_by_key(|m| m.priority());
    }

    /// Spawn principal (pipeline completo)
    pub fn spawn(&mut self, ctx: SpawnContext) -> Result<Pokemon, SpawnError> {
        // 1) tabela base
        let mut table = self.table_provider.table(&ctx);

        // 2) modifiers
        for modifier in &self.modifiers {
            table = modifier.modify(&ctx, &table);
        }

        // 3) spawn engine
        let mut pokemon = self.engine.spawn(&table, &mut self.rng)?;

        // 4) shiny roll
        let chance = self.rarity.shiny_chance_from_context(&ctx);
        if self.rng.float() < chance {
            pokemon.set_shiny(true);
        }

        Ok(pokemon)
    }

    pub fn rng_mut(&mut self) -> &mut R {
        &mut self.rng
    }
}



#[derive(Debug)]
pub enum SpawnError {
    Engine(crate::spawn::engine::SpawnError),
}

impl From<crate::spawn::engine::SpawnError> for SpawnError {
    fn from(e: crate::spawn::engine::SpawnError) -> Self {
        SpawnError::Engine(e)
    }
}