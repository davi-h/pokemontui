use contracts::rng::Rng;

use super::{WeightedEntry, WeightedError};

pub struct WeightedTable<T> {
    entries: Vec<WeightedEntry<T>>,
    total_weight: u32,
}

impl<T> WeightedTable<T> {
    pub fn new(entries: Vec<WeightedEntry<T>>) -> Result<Self, WeightedError> {
        if entries.is_empty() {
            return Err(WeightedError::EmptyTable);
        }

        let total_weight: u32 = entries.iter().map(|e| e.weight).sum();

        if total_weight == 0 {
            return Err(WeightedError::ZeroTotalWeight);
        }

        Ok(Self {
            entries,
            total_weight,
        })
    }

    pub fn choose<'a, R: Rng>(&'a self, rng: &mut R) -> &'a T {
        let mut roll = rng.u32(0, self.total_weight);

        for entry in &self.entries {
            if roll < entry.weight {
                return &entry.item;
            }
            roll -= entry.weight;
        }

        // fallback matematicamente impossível
        &self.entries.last().unwrap().item
    }

    pub fn entries(&self) -> &[WeightedEntry<T>] {
        &self.entries
    }

    pub fn total_weight(&self) -> u32 {
        self.total_weight
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}