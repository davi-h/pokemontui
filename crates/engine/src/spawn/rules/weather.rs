use super::rule::SpawnRule;
use crate::spawn::context::{SpawnContext, Weather};
use crate::spawn::entry::SpawnEntry;

pub struct RainOnly;

impl SpawnRule for RainOnly {
    fn allowed(&self, ctx: &SpawnContext, _entry: &SpawnEntry) -> bool {
        ctx.weather == Weather::Rain
    }
}
