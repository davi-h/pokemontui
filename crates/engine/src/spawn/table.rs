


use crate::spawn::entry::SpawnEntry;

#[derive(Clone)]
pub struct SpawnTable {
    pub biome: String,
    pub entries: Vec<SpawnEntry>,
}


impl SpawnTable {
    pub fn total_weight(&self) -> u32 {
        self.entries.iter().map(|e| e.base_weight).sum()
    }
}
