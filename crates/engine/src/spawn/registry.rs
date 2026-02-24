use super::table::SpawnTable;

pub struct SpawnRegistry {
    table: SpawnTable,
}

impl SpawnRegistry {
    pub fn new(table: SpawnTable) -> Self {
        Self { table }
    }

    pub fn table(&self) -> &SpawnTable {
        &self.table
    }
}