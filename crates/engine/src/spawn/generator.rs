use domain::pokemon::entity::Pokemon;
use contracts::rng::GameRng;

pub struct SpawnGenerator;

impl SpawnGenerator {
    pub fn spawn(mut pokemon: Pokemon, rng: &mut impl GameRng) -> Pokemon {
        let bonus = rng.range(0..5);
        pokemon.stats.attack += bonus as u16;
        pokemon
    }
}