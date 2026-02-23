pub struct EncounterEntry {
    pub species: &'static str,
    pub weight: u32,
    pub min_level: u8,
    pub max_level: u8,
}

pub struct EncounterTable {
    entries: Vec<EncounterEntry>,
    total_weight: u32,
}

impl EncounterTable {
    pub fn new(entries: Vec<EncounterEntry>) -> Self {
        let total_weight = entries.iter().map(|e| e.weight).sum();
        Self { entries, total_weight }
    }

    pub fn pick<'a, R: contracts::rng::Rng>(&'a self, rng: &R) -> &'a EncounterEntry {
        let roll = rng.u32(0, self.total_weight);

        let mut acc = 0;
        for entry in &self.entries {
            acc += entry.weight;
            if roll < acc {
                return entry;
            }
        }

        &self.entries[0]
    }
}