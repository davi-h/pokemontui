#[cfg(test)]
use domain::pokemon::entity::Pokemon;
use crate::factory::pokemon_factory::PokemonFactory;
//use super::encounter_table::EncounterTable; // Classe ainda não criada

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
        &mut self,
        table: &EncounterTable,
        rng: &mut R,
    ) -> Pokemon {
        let entry = table.pick(rng);

        let level = rng.u32(entry.min_level as u32, entry.max_level as u32 + 1) as u8;

        self.factory
            .create(entry.species, level)
            .expect("failed to create pokemon from factory")
    }
}