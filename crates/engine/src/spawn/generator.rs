use domain::pokemon::entity::Pokemon;
use crate::rng::GameRng;

pub struct SpawnGenerator;

impl SpawnGenerator {
    pub fn spawn(mut pokemon: Pokemon) -> Pokemon {
        let bonus = GameRng::range(0, 5) as u16;
        pokemon.stats.attack += bonus;
        pokemon
    }
}