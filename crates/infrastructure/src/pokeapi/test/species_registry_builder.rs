use std::collections::HashSet;

use contracts::api::{pokemon_data_source::PokemonDataSource, error::ApiError};
use crate::{cache::file_cache::FileCache, sprites::sprite_loader::SpriteLoader};
use engine::factory::species_registry::SpeciesRegistry;

pub struct SpeciesRegistryBuilder;

impl SpeciesRegistryBuilder {
    pub fn build<D>(
        data: &D,
        sprites: &SpriteLoader,
        cache: &FileCache,
    ) -> Result<SpeciesRegistry, ApiError>
    where
        D: PokemonDataSource,
    {
        // cache
        if let Some(cached) = cache.get("species.txt") {
            return Ok(SpeciesRegistry::new(
                    cached
                        .lines()
                        .map(|s| s.to_string())
                        .collect::<std::collections::HashSet<String>>()
                )
            )
        }

        let mut set = HashSet::new();

        let seed_list = [
            "bulbasaur","ivysaur","venusaur",
            "charmander","charmeleon","charizard",
            "squirtle","wartortle","blastoise",
            "pikachu","raichu",
        ];

        for name in seed_list {
            if !sprites.exists(name) {
                continue;
            }

            if data.fetch(name).is_ok() {
                set.insert(name.to_string());
            }
        }

        let serialized = set.iter().cloned().collect::<Vec<_>>().join("\n");
        cache.set("species.txt", &serialized);

        Ok(SpeciesRegistry::new(set))
    }
}