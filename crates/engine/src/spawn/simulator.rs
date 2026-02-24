use std::collections::HashMap;
use super::{engine::SpawnEngine, registry::SpawnRegistry, context::SpawnContext};

pub fn simulate(
    engine: &mut SpawnEngine,
    ctx: &SpawnContext,
    reg: &SpawnRegistry,
    rolls: u32,
) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    for _ in 0..rolls {
        if let Some(r) = engine.spawn(ctx, reg) {
            *map.entry(r.species.clone()).or_insert(0) += 1;
        }
    }

    map
}