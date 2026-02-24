use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApiPokemon {
    pub name: String,
    pub stats: Vec<ApiStat>,
}

#[derive(Deserialize)]
pub struct ApiStat {
    pub base_stat: u16,
    pub stat: ApiStatName,
}

#[derive(Deserialize)]
pub struct ApiStatName {
    pub name: String,
}