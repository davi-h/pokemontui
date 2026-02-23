
use super::stats::Stats;

#[derive(Clone)]
pub struct Pokemon {
    pub name: String,
    pub level: u8,
    pub stats: Stats,
}