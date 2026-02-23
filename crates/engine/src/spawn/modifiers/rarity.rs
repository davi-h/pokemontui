use super::modifier::WeightModifier;
use crate::spawn::{context::SpawnContext, entry::SpawnEntry};

pub struct RareBoost;

impl WeightModifier for RareBoost {
    fn modify(&self, _ctx: &SpawnContext, entry: &SpawnEntry, weight: u32) -> u32 {
        if entry.base_weight < 10 {
            weight * 2
        } else {
            weight
        }
    }
}
