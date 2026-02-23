use crate::spawn::{context::SpawnContext, entry::SpawnEntry};

pub trait WeightModifier {
    fn modify(&self, ctx: &SpawnContext, entry: &SpawnEntry, weight: u32) -> u32;
}
