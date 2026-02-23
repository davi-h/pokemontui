//! Shared domain models used across the application.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub types: Vec<String>,
    // more fields will be added based on API
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Move {
    pub id: u32,
    pub name: String,
    pub power: Option<u32>,
    // etc.
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
}

/// Entry in the pokédex, may include a local sprite path once fetched.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PokedexEntry {
    pub pokemon: Pokemon,
    pub sprite_path: Option<String>,
    pub sprite_ascii: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    // basic skeleton
    pub pokedex: Vec<PokedexEntry>,
    // inventory, player stats, etc.
}
