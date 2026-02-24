use super::{
    context::SpawnContext,
    registry::SpawnRegistry,
    rng::SeededRng,
    rules::rule::SpawnRule,
    modifiers::modifier::WeightModifier,
};

use contracts::rng::Rng;

/// Motor principal responsável por calcular qual Pokémon spawnar
pub struct SpawnEngine {
    rng: SeededRng,
    rules: Vec<Box<dyn SpawnRule>>,
    modifiers: Vec<Box<dyn WeightModifier>>,
}

/// Resultado final de um spawn
#[derive(Debug, Clone)]
pub struct SpawnResult {
    pub species: String,
    pub level: u8,
}

impl SpawnEngine {
    /// Cria engine com seed determinística
    pub fn new(seed: u64) -> Self {
        Self {
            rng: SeededRng::new(seed),
            rules: Vec::new(),
            modifiers: Vec::new(),
        }
    }

    /// Adiciona regra de spawn
    pub fn add_rule<R>(&mut self, rule: R)
    where
        R: SpawnRule + 'static,
    {
        self.rules.push(Box::new(rule));
    }

    /// Adiciona modificador de peso
    pub fn add_modifier<M>(&mut self, modifier: M)
    where
        M: WeightModifier + 'static,
    {
        self.modifiers.push(Box::new(modifier));
    }

    /// Executa cálculo de spawn
    pub fn spawn(
        &mut self,
        ctx: &SpawnContext,
        registry: &SpawnRegistry,
    ) -> Option<SpawnResult> {
        let table = registry.table();

        let mut pool: Vec<(_, u32)> = Vec::with_capacity(table.entries.len());

        for entry in &table.entries {
            if entry.min_level > entry.max_level {
                continue;
            }

            // regras
            if self.rules.iter().all(|r| r.allowed(ctx, entry)) {
                let mut weight = entry.base_weight;

                // modificadores
                for m in &self.modifiers {
                    weight = m.modify(ctx, entry, weight).max(0);
                }

                if weight > 0 {
                    pool.push((entry, weight));
                }
            }
        }

        let total: usize = pool.iter().map(|(_, w)| *w as usize).sum();

        if total == 0 {
            return None;
        }

        let roll = self.rng.range(0..total);

        let mut acc: usize = 0;

        for (entry, weight) in pool {
            acc += weight as usize;

            if roll < acc {
                let lvl = self.rng.range(
                    entry.min_level as usize..entry.max_level as usize + 1
                ) as u8;

                return Some(SpawnResult {
                    species: entry.species.clone(),
                    level: lvl,
                });
            }
        }

        None
    }
}