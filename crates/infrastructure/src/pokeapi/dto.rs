use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PokemonDTO {
    pub name: String,
    pub base_experience: u32,
    pub stats: Vec<StatEntry>,
    pub types: Vec<TypeEntry>,
}

#[derive(Debug, Deserialize)]
pub struct StatEntry {
    pub base_stat: u32,
    pub stat: StatName,
}

#[derive(Debug, Deserialize)]
pub struct StatName {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TypeEntry {
    #[serde(rename = "type")]
    pub type_info: TypeName,
}

#[derive(Debug, Deserialize)]
pub struct TypeName {
    pub name: String,
}