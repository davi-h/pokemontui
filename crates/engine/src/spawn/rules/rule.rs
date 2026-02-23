use crate::spawn::context::SpawnContext;
use crate::spawn::entry::SpawnEntry;

pub trait SpawnRule {
    fn allowed(&self, ctx: &SpawnContext, entry: &SpawnEntry) -> bool;
}
