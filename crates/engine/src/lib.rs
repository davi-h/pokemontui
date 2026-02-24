//! NOTE: All infrastructure dependencies must go through contracts traits, not direct dependencies, to prevent cycles.
pub mod ai;
pub mod battle;
pub mod core;
pub mod dex;
pub mod factory;
pub mod pokedex;
pub mod render;
pub mod spawn;
pub mod stats;
pub mod turn;
pub mod util;
pub mod test;
