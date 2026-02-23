
use super::stats::Stats;

#[derive(Clone)]
pub struct Pokemon {
    pub name: String,
    pub level: u8,
    pub stats: Stats,
}
impl Pokemon {
    pub fn new(name: String, level: u8) -> Self {
        Self {
            name,
            level,
            stats: Stats::from_level(level),
        }
    }
}