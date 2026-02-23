use std::collections::HashMap;
use super::table::SpawnTable;

pub struct SpawnRegistry {
    tables: HashMap<String, SpawnTable>,
}

impl SpawnRegistry {
    pub fn new() -> Self {
        Self { tables: HashMap::new() }
    }

    pub fn register(&mut self, table: SpawnTable) {
        self.tables.insert(table.biome.clone(), table);
    }

    pub fn get(&self, biome: &str) -> Option<&SpawnTable> {
        self.tables.get(biome)
    }
}
