use crate::api::error::ApiError;

#[derive(Debug, Clone)]
pub struct PokemonApiData {
    pub name: String,
    pub base_stats: BaseStats,
    pub types: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BaseStats {
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub special_attack: u16,
    pub special_defense: u16,
    pub speed: u16,
}

pub trait PokemonDataSource {
    fn fetch(&self, name: &str) -> Result<PokemonApiData, ApiError>;
}