pub struct SpawnService<F, R, E>
where
    F: PokemonFactory,
    R: Rng,
    E: EnvironmentProvider,
{
    factory: F,
    rng: R,
    rarity: RarityEngine,
    env: E,
}

impl<F, R> SpawnService<F, R>
where
    F: PokemonFactory,
    R: contracts::rng::Rng,
{
    pub fn spawn(&mut self, level: u8) -> Pokemon {
        let mut pokemon = self.factory.create_random(level);

        let environment = self.env.current();
        let chance = self.rarity.shiny_chance(&environment);

        if self.rng.f32() < chance {
            pokemon.set_shiny(true);
        }

    pokemon
    }
}