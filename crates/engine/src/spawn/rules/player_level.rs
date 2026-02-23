use super::rule::SpawnRule;
use crate::spawn::context::SpawnContext;
use crate::spawn::entry::SpawnEntry;

pub struct MinPlayerLevel(pub u8);

impl SpawnRule for MinPlayerLevel {
    fn allowed(&self, ctx: &SpawnContext, _entry: &SpawnEntry) -> bool {
        ctx.player_level >= self.0
    }
}
