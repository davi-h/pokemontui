use super::rule::SpawnRule;
use crate::spawn::context::{SpawnContext, TimeOfDay};
use crate::spawn::entry::SpawnEntry;

pub struct NightOnly;

impl SpawnRule for NightOnly {
    fn allowed(&self, ctx: &SpawnContext, _entry: &SpawnEntry) -> bool {
        ctx.time_of_day == TimeOfDay::Night
    }
}
