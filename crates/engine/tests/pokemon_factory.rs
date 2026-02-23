use engine::factory::pokemon_factory::{DefaultPokemonFactory, PokemonFactory};
use contracts::rng::Rng;

struct FakeRng;

impl Rng for FakeRng {
    fn u32(&self, _: u32, _: u32) -> u32 { 10 }
}

#[test]
fn factory_creates_predictable_stats() {
    let factory = DefaultPokemonFactory::new(FakeRng);

    let p = factory.create("pikachu", 5);

    assert_eq!(p.stats.hp, 10);
}