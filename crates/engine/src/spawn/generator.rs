use contracts::rng::Rng;
use domain::pokemon::entity::Pokemon;
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
use contracts::rng::GameRng;
=======
use crate::rng::GameRng; // falta essa classe aqui
>>>>>>> 8278913 (v0.0.3)
=======
use contracts::rng::GameRng;
>>>>>>> 694a416 (v0.0.4)
=======
>>>>>>> 9d24c78 (v0.0.5)

/// Gerador de spawn determinístico.
///
/// Regras:
/// - depende apenas de contrato RNG
/// - não conhece implementação concreta
/// - reproduzível com seed fixa
pub struct SpawnGenerator<'a> {
    rng: R,
}

<<<<<<< HEAD
impl SpawnGenerator {
<<<<<<< HEAD
    pub fn spawn_with_rng(mut pokemon: Pokemon, rng: &mut dyn GameRng) -> Pokemon {
        let bonus = rng.range_u32(0..5) as u16;
        pokemon.stats.attack += bonus;
=======
    pub fn spawn(mut pokemon: Pokemon, rng: &mut impl GameRng) -> Pokemon {
        let bonus = rng.range(0..5);
        pokemon.stats.attack += bonus as u16;
>>>>>>> 694a416 (v0.0.4)
        pokemon
    }
}
=======

impl<'a> SpawnGenerator<'a> {
    pub fn new(rng: R) -> Self
}
>>>>>>> 9d24c78 (v0.0.5)
