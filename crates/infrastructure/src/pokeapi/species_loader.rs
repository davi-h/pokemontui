use std::{
    fs,
    path::PathBuf,
    process::Command,
    sync::{Arc, Mutex},
    time::Duration,
};

use serde::{Deserialize, Serialize};
use rayon::prelude::*;

use contracts::api::{PokemonDataSource};
use contracts::api::error::ApiError;

#[derive(Serialize, Deserialize)]
struct SpeciesCache {
    species: Vec<String>,
}

pub struct SpeciesLoader<D>
where
    D: PokemonDataSource + Send + Sync + Clone + 'static,
{
    cache_path: PathBuf,
    data_source: D,
    retries: usize,
}

impl<D> SpeciesLoader<D>
where
    D: PokemonDataSource + Send + Sync + Clone + 'static,
{
    pub fn new(cache_path: impl Into<PathBuf>, data_source: D) -> Self {
        Self {
            cache_path: cache_path.into(),
            data_source,
            retries: 2,
        }
    }

    pub fn with_retries(mut self, retries: usize) -> Self {
        self.retries = retries;
        self
    }

    pub fn load(&self) -> Result<Vec<String>, ApiError> {
        if self.cache_path.exists() {
            return self.load_cache();
        }

        println!("Generating species list (first run, may take a moment)…");

        let list = self.generate()?;
        self.save_cache(&list)?;

        Ok(list)
    }

    fn generate(&self) -> Result<Vec<String>, ApiError> {
        let names = self.list_from_pokeget()?;

        let total = names.len();
        let counter = Arc::new(Mutex::new(0usize));

        let valid: Vec<String> = names
            .par_iter()
            .filter_map(|name| {
                let ok = self.try_fetch(name);

                let mut c = counter.lock().unwrap();
                *c += 1;
                if *c % 25 == 0 || *c == total {
                    println!("validated {}/{}", *c, total);
                }

                if ok { Some(name.clone()) } else { None }
            })
            .collect();

        if valid.is_empty() {
            return Err(ApiError::Parse("no valid species found".into()));
        }

        Ok(valid)
    }

    fn try_fetch(&self, name: &str) -> bool {
        for _ in 0..=self.retries {
            if self.data_source.fetch(name).is_ok() {
                return true;
            }

            std::thread::sleep(Duration::from_millis(120));
        }
        false
    }

    fn list_from_pokeget(&self) -> Result<Vec<String>, ApiError> {
        let output = Command::new("pokeget")
            .arg("--list")
            .output()
            .map_err(|e| ApiError::Network(e.to_string()))?;

        if !output.status.success() {
            return Err(ApiError::Parse("pokeget failed".into()));
        }

        let text = String::from_utf8_lossy(&output.stdout);

        let mut list: Vec<String> = text
            .lines()
            .map(|l| l.trim().to_lowercase())
            .filter(|l| !l.is_empty())
            .collect();

        list.sort();
        list.dedup();

        Ok(list)
    }

    fn save_cache(&self, list: &[String]) -> Result<(), ApiError> {
        let tmp = self.cache_path.with_extension("tmp");

        let json = serde_json::to_string(&SpeciesCache {
            species: list.to_vec(),
        })
        .map_err(|e| ApiError::Parse(e.to_string()))?;

        fs::write(&tmp, json)
            .map_err(|e| ApiError::Network(e.to_string()))?;

        fs::rename(tmp, &self.cache_path)
            .map_err(|e| ApiError::Network(e.to_string()))?;

        Ok(())
    }

    fn load_cache(&self) -> Result<Vec<String>, ApiError> {
        let text = fs::read_to_string(&self.cache_path)
            .map_err(|e| ApiError::Network(e.to_string()))?;

        let data: SpeciesCache =
            serde_json::from_str(&text)
                .map_err(|e| ApiError::Parse(e.to_string()))?;

        Ok(data.species)
    }
}