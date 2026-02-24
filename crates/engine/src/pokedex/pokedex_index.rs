use serde::{Serialize, Deserialize};
use contracts::api::pokemon_data_source::PokemonDataSource;
use infrastructure::cache::file_cache::FileCache;
use infrastructure::sprites::sprite_loader::SpriteLoader;
use contracts::api::error::ApiError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DexEntry {
    pub id: u16,
    pub name: String,
}

#[derive(Clone)]
pub struct PokedexIndex {
    entries: Vec<DexEntry>,
}

impl PokedexIndex {
    pub fn load<D>(
        api: &D,
        sprites: &SpriteLoader,
        cache: &FileCache,
    ) -> Result<Self, ApiError>
    where
        D: PokemonDataSource,
    {
        if let Some(json) = cache.get("pokedex.json") {
            let entries: Vec<DexEntry> = serde_json::from_str(&json).unwrap();
            return Ok(Self { entries });
        }

        let mut list = Vec::new();

        for id in 1..=898 {
            let name = id.to_string();

            // API valida
            let data = match api.fetch(&name) {
                Ok(d) => d,
                Err(_) => continue,
            };

            // sprite valida
            if !sprites.exists(&data.name) {
                continue;
            }

            list.push(DexEntry {
                id: data.id as u16,
                name: data.name,
            });
        }

        let json = serde_json::to_string_pretty(&list).unwrap();
        cache.set("pokedex.json", &json);

        Ok(Self { entries: list })
    }

    pub fn get(&self, id: u16) -> Option<&DexEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    pub fn all(&self) -> &[DexEntry] {
        &self.entries
    }

    pub fn random<R: contracts::rng::Rng>(&self, rng: &mut R) -> &DexEntry {
        let idx = rng.u32(0, self.entries.len() as u32) as usize;
        &self.entries[idx]
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}