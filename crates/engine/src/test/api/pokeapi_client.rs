use std::fs;
use std::path::Path;

use serde::Deserialize;

use domain::pokemon::stats::Stats;

const BASE_URL: &str = "https://pokeapi.co/api/v2/pokemon";

pub struct PokeApiClient;

impl PokeApiClient {
    pub fn new() -> Self {
        Self
    }

    pub fn fetch_stats(&self, id: u16) -> Result<Stats, String> {
        let json = self.load_or_fetch(id)?;
        let parsed: ApiPokemon = serde_json::from_str(&json)
            .map_err(|e| format!("JSON parse error: {e}"))?;

        Ok(parsed.into_stats())
    }

    fn load_or_fetch(&self, id: u16) -> Result<String, String> {
        let path = format!("assets/cache/pokemon_{id}.json");

        if Path::new(&path).exists() {
            return fs::read_to_string(&path)
                .map_err(|e| format!("Cache read error: {e}"));
        }

        let url = format!("{BASE_URL}/{id}");

        let body = reqwest::blocking::get(&url)
            .map_err(|e| format!("Request error: {e}"))?
            .text()
            .map_err(|e| format!("Body read error: {e}"))?;

        fs::create_dir_all("assets/cache")
            .map_err(|e| format!("Cache dir error: {e}"))?;

        fs::write(&path, &body)
            .map_err(|e| format!("Cache write error: {e}"))?;

        Ok(body)
    }
}

#[derive(Deserialize)]
struct ApiPokemon {
    stats: Vec<ApiStat>,
}

#[derive(Deserialize)]
struct ApiStat {
    base_stat: u16,
    stat: StatName,
}

#[derive(Deserialize)]
struct StatName {
    name: String,
}

impl ApiPokemon {
    fn into_stats(self) -> Stats {
        let mut hp = 0;
        let mut atk = 0;
        let mut def = 0;
        let mut spa = 0;
        let mut spd = 0;
        let mut spe = 0;

        for s in self.stats {
            match s.stat.name.as_str() {
                "hp" => hp = s.base_stat,
                "attack" => atk = s.base_stat,
                "defense" => def = s.base_stat,
                "special-attack" => spa = s.base_stat,
                "special-defense" => spd = s.base_stat,
                "speed" => spe = s.base_stat,
                _ => {}
            }
        }

        Stats::from_base(hp, atk, def, spa, spd, spe)
    }
}