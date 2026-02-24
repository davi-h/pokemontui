//! Game systems such as rarity calculation and trait generation.

use super::models::*;

pub struct Rarity {
    pub capture_rate: f32,
    pub spawn_weight: f32,
}

pub trait RaritySystem {
    fn calculate(&self, pokemon: &Pokemon) -> Rarity;
}

/// Placeholder basic rarity system based on id or other heuristics.
pub struct BasicRarity;
impl RaritySystem for BasicRarity {
    fn calculate(&self, _pokemon: &Pokemon) -> Rarity {
        // naive stub
        Rarity { capture_rate: 0.5, spawn_weight: 1.0 }
    }
}

pub struct Traits {
    pub iv_attack: u8,
    pub iv_defense: u8,
    // etc
}

pub trait TraitGenerator {
    fn generate(&self, pokemon: &Pokemon) -> Traits;
}

pub struct BasicTraitGenerator;
impl TraitGenerator for BasicTraitGenerator {
    fn generate(&self, _pokemon: &Pokemon) -> Traits {
        Traits { iv_attack: 0, iv_defense: 0 }
    }
}
