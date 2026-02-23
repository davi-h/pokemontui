use serde::Deserialize;

#[derive(Deserialize)]
pub struct PokemonDTO {
    pub name: String,
    pub base_experience: u32,
}