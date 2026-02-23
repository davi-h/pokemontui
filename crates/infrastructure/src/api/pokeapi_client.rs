impl Default for PokeApiClient {
    fn default() -> Self {
        Self::new()
    }
}
use contracts::api::PokemonApi;
use super::dto::PokemonDTO;

pub struct PokeApiClient {
    client: reqwest::blocking::Client,
}

impl PokeApiClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl PokemonApi for PokeApiClient {
    fn fetch(&self, name: &str) -> String {
        let url = format!("https://pokeapi.co/api/v2/pokemon/{name}");

        let res = self.client.get(url).send().unwrap();
        let dto: PokemonDTO = res.json().unwrap();

        dto.name
    }
}