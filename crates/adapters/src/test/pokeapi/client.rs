use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct PokemonList {
    results: Vec<PokemonName>
}

#[derive(Deserialize)]
struct PokemonName {
    name: String
}

pub struct PokeApiClient {
    client: Client
}

impl PokeApiClient {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }

    pub fn fetch(&self, offset: usize, limit: usize) -> Vec<String> {
        let url = format!(
            "https://pokeapi.co/api/v2/pokemon?offset={}&limit={}",
            offset, limit
        );

        let res: PokemonList = self.client.get(url).send().unwrap().json().unwrap();
        res.results.into_iter().map(|p| p.name).collect()
    }
    pub async fn fetch_species(name: &str) -> anyhow::Result<String> {
        let url = format!("https://pokeapi.co/api/v2/pokemon/{name}");
        let body = reqwest::get(url).await?.text().await?;
        Ok(body)
}
}