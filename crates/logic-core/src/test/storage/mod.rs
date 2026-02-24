//! Storage layer handling saves and cache.

use super::models::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum StorageError {
    Io(std::io::Error),
    Serde(serde_json::Error),
    NotFound,
}

/// Abstraction for persistent game state and cache.
pub trait Storage {
    fn load_save(&self) -> Result<GameState, StorageError>;
    fn save_state(&self, state: &GameState) -> Result<(), StorageError>;

    fn cache_pokemon(&mut self, p: Pokemon);
    fn get_cached_pokemon(&self, name: &str) -> Option<Pokemon>;
}

/// Simple in-memory store implementation (for tests/prototyping)
pub struct InMemoryStorage {
    pub state: GameState,
    pub cache: HashMap<String, Pokemon>,
}

impl Storage for InMemoryStorage {
    fn load_save(&self) -> Result<GameState, StorageError> {
        Ok(self.state.clone())
    }
    fn save_state(&self, _state: &GameState) -> Result<(), StorageError> {
        // no-op
        Ok(())
    }
    fn cache_pokemon(&mut self, p: Pokemon) {
        self.cache.insert(p.name.clone(), p);
    }
    fn get_cached_pokemon(&self, name: &str) -> Option<Pokemon> {
        self.cache.get(name).cloned()
    }
}
