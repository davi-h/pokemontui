use contracts::api::{PokemonDataSource, PokemonApiData, BaseStats};
use contracts::api::error::ApiError;

use super::dto::PokemonDTO;

pub struct PokeApiClient {
    client: reqwest::blocking::Client,
}

impl Default for PokeApiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl PokeApiClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .map_err(|e| ApiError::Network(e.to_string()))
                .expect("failed to build HTTP client"),
        }
    }
}

impl PokemonDataSource for PokeApiClient {
    fn fetch(&self, name: &str) -> Result<PokemonApiData, ApiError> {
        let url = format!("https://pokeapi.co/api/v2/pokemon/{}", name.to_lowercase());

        let res = self.client
            .get(url)
            .send()
            .map_err(|e| ApiError::Network(e.to_string()))?;

        if !res.status().is_success() {
            return Err(ApiError::Http(res.status().as_u16()));
        }

        let dto: PokemonDTO = res
            .json()
            .map_err(|e| ApiError::Parse(e.to_string()))?;

        // ---------- stats ----------
        let mut hp = None;
        let mut attack = None;
        let mut defense = None;
        let mut sp_atk = None;
        let mut sp_def = None;
        let mut speed = None;

        for s in dto.stats {
            match s.stat.name.as_str() {
                "hp" => hp = Some(s.base_stat),
                "attack" => attack = Some(s.base_stat),
                "defense" => defense = Some(s.base_stat),
                "special-attack" => sp_atk = Some(s.base_stat),
                "special-defense" => sp_def = Some(s.base_stat),
                "speed" => speed = Some(s.base_stat),
                _ => {}
            }
        }

        fn stat(v: Option<u32>, name: &str) -> Result<u16, ApiError> {
            v.ok_or_else(|| ApiError::Parse(format!("missing {name} stat")))?
                .try_into()
                .map_err(|_| ApiError::Parse(format!("{name} overflow")))
        }

        let stats = BaseStats {
            hp: stat(hp, "hp")?,
            attack: stat(attack, "attack")?,
            defense: stat(defense, "defense")?,
            special_attack: stat(sp_atk, "special_attack")?,
            special_defense: stat(sp_def, "special_defense")?,
            speed: stat(speed, "speed")?,
        };

        // ---------- types ----------
        let types = dto
            .types
            .into_iter()
            .map(|t| t.type_info.name)
            .collect();

        Ok(PokemonApiData {
            name: dto.name,
            base_stats: stats,
            types,
        })
    }
}