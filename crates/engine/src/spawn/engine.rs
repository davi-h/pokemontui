use super::{
    context::SpawnContext,
    registry::SpawnRegistry,
    rng::SeededRng,
    rules::rule::SpawnRule,
    modifiers::modifier::WeightModifier,
};

pub struct SpawnEngine {
    rng: SeededRng,
    rules: Vec<Box<dyn SpawnRule>>,
    modifiers: Vec<Box<dyn WeightModifier>>,
}

pub struct SpawnResult {
    pub species: String,
    pub level: u8,
}

impl SpawnEngine {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: SeededRng::new(seed),
            rules: vec![],
            modifiers: vec![],
        }
    }

    pub fn add_rule<R: SpawnRule + 'static>(&mut self, rule: R) {
        self.rules.push(Box::new(rule));
    }

    pub fn add_modifier<M: WeightModifier + 'static>(&mut self, m: M) {
        self.modifiers.push(Box::new(m));
    }

    pub fn spawn(
        &mut self,
        ctx: &SpawnContext,
        registry: &SpawnRegistry,
    ) -> Option<SpawnResult> {
        let table = registry.get(&ctx.biome)?;

        let mut pool = vec![];

        for entry in &table.entries {
            if self.rules.iter().all(|r| r.allowed(ctx, entry)) {
                let mut weight = entry.base_weight;

                for m in &self.modifiers {
                    weight = m.modify(ctx, entry, weight);
                }

                pool.push((entry, weight));
            }
        }

        let total: u32 = pool.iter().map(|(_, w)| *w).sum();
        if total == 0 { return None; }

        let roll = self.rng.range(0, total);

        let mut acc = 0;
        for (entry, w) in pool {
            acc += w;
            if roll < acc {
                let lvl = self.rng.range(entry.min_level as u32, entry.max_level as u32 + 1) as u8;
                return Some(SpawnResult {
                    species: entry.species.clone(),
                    level: lvl,
                });
            }
        }

        None
    }
}
