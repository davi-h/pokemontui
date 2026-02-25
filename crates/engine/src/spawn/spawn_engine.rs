use domain::pokemon::entity::Pokemon;

use crate::factory::pokemon_factory::{PokemonFactory, FactoryError};
use crate::spawn::distribution::WeightedDistribution;
use crate::spawn::modifiers::SpawnModifier;
use crate::spawn::table::SpawnTable;

/// Motor canônico de spawn — pipeline funcional puro
///
/// Pipeline:
/// Table -> Modifiers -> Distribution -> Roll -> Factory
///
/// Regras:
/// ✔ Determinístico (RNG injetado)
/// ✔ Sem regras globais internas
/// ✔ Sem estado interno
/// ✔ Sem side-effects
///
/// Toda lógica de mundo deve vir via modifiers.
pub struct SpawnEngine<F> {
    factory: F,
}

impl<F> SpawnEngine<F> {
    pub fn new(factory: F) -> Self {
        Self { factory }
    }

    pub fn spawn<R>(
        &mut self,
        base_table: &SpawnTable,
        modifiers: &[&dyn SpawnModifier],
        rng: &mut R,
    ) -> Result<Pokemon, SpawnError>
    where
        R: contracts::rng::Rng,
        F: PokemonFactory,
    {
        // ─────────────────────────────
        // Stage 1 — apply modifiers
        // ─────────────────────────────
        let mut table = base_table.clone();

        for modifier in modifiers {
            table = modifier.modify(&table);
        }

        // ─────────────────────────────
        // Stage 2 — distribution pick
        // ─────────────────────────────
        let entry = WeightedDistribution::pick(&table.entries, rng)
            .ok_or(SpawnError::EmptyTable)?;

        // ─────────────────────────────
        // Stage 3 — level roll
        // ─────────────────────────────
        let level = rng.u32(entry.min_level as u32, entry.max_level as u32 + 1) as u8;

        // ─────────────────────────────
        // Stage 4 — factory
        // ─────────────────────────────
        self.factory
            .create(&entry.species, level)
            .map_err(SpawnError::Factory)
    }
}

#[derive(Debug)]
pub enum SpawnError {
    EmptyTable,
    Factory(FactoryError),
}