use super::rng::SeededRng;
use super::table::{SpawnEntry, SpawnTable};

#[derive(Clone)]
pub struct SpawnResult {
    pub species: String,
    pub level: u8,
}

pub struct SpawnEngine {
    rng: SeededRng,
}

impl SpawnEngine {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: SeededRng::new(seed),
        }
    }

    pub fn spawn(&mut self, table: &SpawnTable) -> Option<SpawnResult> {
        if table.entries.is_empty() {
            return None;
        }

        let roll = self.rng.range(0, table.total_weight());

        let mut acc = 0;

        for entry in &table.entries {
            acc += entry.weight;

            if roll < acc {
                return Some(self.build_result(entry));
            }
        }

        None
    }

    fn build_result(&mut self, entry: &SpawnEntry) -> SpawnResult {
        let level = self
            .rng
            .range(entry.min_level as u32, entry.max_level as u32 + 1) as u8;

        SpawnResult {
            species: entry.species.clone(),
            level,
        }
    }
}
