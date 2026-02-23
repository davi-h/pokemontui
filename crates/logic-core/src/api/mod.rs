impl Default for ReqwestClient {
    fn default() -> Self {
        Self::new()
    }
}
/// Core API abstractions for interfacing with external services (PokéAPI, pokeget, etc.)
use crate::models::*;
use std::process::Command;

/// Error type returned by API operations.
#[derive(Debug)]
pub enum ApiError {
    Http(reqwest::Error),
    Io(std::io::Error),
    NotFound,
    Other(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Http(e) => write!(f, "HTTP error: {}", e),
            ApiError::Io(e) => write!(f, "IO error: {}", e),
            ApiError::NotFound => write!(f, "Not found"),
            ApiError::Other(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for ApiError {}

/// Trait providing methods to fetch Pokémon-related data.
pub trait ApiClient {
    fn fetch_pokemon(&self, name: &str) -> Result<Pokemon, ApiError>;
    fn fetch_move(&self, id: u16) -> Result<Move, ApiError>;
    fn fetch_pokemon_list(&self) -> Result<Vec<String>, ApiError>;
    fn fetch_sprite_ascii(&self, name: &str) -> Result<String, ApiError>;
}

/// Concrete client using reqwest blocking API and pokeget command.
pub struct ReqwestClient {
    base_url: String,
}

impl ReqwestClient {
    pub fn new() -> Self {
        ReqwestClient {
            base_url: "https://pokeapi.co/api/v2".to_string(),
        }
    }
}

impl ApiClient for ReqwestClient {
    fn fetch_pokemon(&self, name: &str) -> Result<Pokemon, ApiError> {
        let url = format!("{}/pokemon/{}", self.base_url, name);
        let resp = reqwest::blocking::get(&url).map_err(ApiError::Http)?;
        if resp.status().is_success() {
            let p = resp.json::<Pokemon>().map_err(ApiError::Http)?;
            Ok(p)
        } else {
            Err(ApiError::NotFound)
        }
    }

    fn fetch_move(&self, id: u16) -> Result<Move, ApiError> {
        let url = format!("{}/move/{}", self.base_url, id);
        let resp = reqwest::blocking::get(&url).map_err(ApiError::Http)?;
        if resp.status().is_success() {
            let m = resp.json::<Move>().map_err(ApiError::Http)?;
            Ok(m)
        } else {
            Err(ApiError::NotFound)
        }
    }

    fn fetch_pokemon_list(&self) -> Result<Vec<String>, ApiError> {
        let url = format!("{}/pokemon?limit=10000", self.base_url);
        let resp = reqwest::blocking::get(&url).map_err(ApiError::Http)?;
        #[derive(serde::Deserialize)]
        struct ListResponse {
            results: Vec<NamedResource>,
        }
        #[derive(serde::Deserialize)]
        #[allow(dead_code)]
        struct NamedResource {
            name: String,
            url: String,
        }
        let list = resp.json::<ListResponse>().map_err(ApiError::Http)?;
        Ok(list.results.into_iter().map(|r| r.name).collect())
    }

    fn fetch_sprite_ascii(&self, name: &str) -> Result<String, ApiError> {
        // pokeget prints ASCII sprite to stdout
        let output = Command::new("pokeget").arg(name).output().map_err(ApiError::Io)?;
        if output.status.success() {
            let ascii = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(ascii)
        } else {
            Err(ApiError::Other(format!("pokeget failed: {}", name)))
        }
    }
}
