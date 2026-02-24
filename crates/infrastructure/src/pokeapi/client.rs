use contracts::api::pokemon_data_source::{
    PokemonDataSource, PokemonApiData, BaseStats,
};
use contracts::api::error::ApiError;

#[derive(Clone)]
pub struct PokeApiClient;

impl PokeApiClient {
    pub fn new() -> Self {
        Self
    }
}

impl PokemonDataSource for PokeApiClient {
    fn fetch(&self, name: &str) -> Result<PokemonApiData, ApiError> {
        // stub temporário
        Ok(PokemonApiData {
            name: name.to_string(),
            base_stats: BaseStats {
                hp: 50,
                attack: 50,
                defense: 50,
                special_attack: 50,
                special_defense: 50,
                speed: 50,
            },
            types: vec!["normal".into()],
        })
    }
}