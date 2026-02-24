use domain::pokemon::entity::Pokemon;
<<<<<<< HEAD
use contracts::rng::GameRng;
=======
use crate::rng::GameRng; // falta essa classe aqui
>>>>>>> 8278913 (v0.0.3)

pub struct SpawnGenerator;

impl SpawnGenerator {
    pub fn spawn_with_rng(mut pokemon: Pokemon, rng: &mut dyn GameRng) -> Pokemon {
        let bonus = rng.range_u32(0..5) as u16;
        pokemon.stats.attack += bonus;
        pokemon
    }
}
