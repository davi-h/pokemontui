use super::rng::SeededRng;
use super::table::{SpawnEntry, SpawnTable};
use contracts::rng::Rng;

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

        let total = table.total_weight();
        if total == 0 {
            return None;
        }

        let roll = self.rng.range(0..total as usize);

        let mut acc: u32 = 0;

        for entry in &table.entries {
            acc += entry.base_weight;

            if roll < acc as usize{
                return Some(self.build_result(entry));
            }
        }

        None
    }

    fn build_result(&mut self, entry: &SpawnEntry) -> SpawnResult {
        let level = self
            .rng
            .range(entry.min_level as usize..(entry.max_level as usize + 1)) as u8;

        SpawnResult {
            species: entry.species.clone(),
            level,
        }
    }
}