//! NOTE: All infrastructure dependencies must go through contracts traits, not direct dependencies, to prevent cycles.
pub mod battle;
pub mod turn;
pub mod rng;
pub mod spawn;
pub mod stats;
pub mod factory;
