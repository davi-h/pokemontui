use domain::pokemon::entity::Pokemon;
use crate::factory::PokemonFactory;
use super::encounter_table::EncounterTable;

pub struct SpawnEngine<F> {
    factory: F,
}

impl<F> SpawnEngine<F> {
    pub fn new(factory: F) -> Self {
        Self { factory }
    }
}

impl<F: PokemonFactory> SpawnEngine<F> {
    pub fn spawn<R: contracts::rng::Rng>(
        &self,
        table: &EncounterTable,
        rng: &R,
    ) -> Pokemon {
        let entry = table.pick(rng);

        let level = rng.u32(entry.min_level as u32, entry.max_level as u32 + 1) as u8;

        self.factory.create(entry.species, level).unwrap()
    }
}