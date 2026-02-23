use engine::factory::pokemon_factory::PokemonFactory;

pub struct SpawnPokemon<F: PokemonFactory> {
    factory: F,
}

impl<F: PokemonFactory> SpawnPokemon<F> {
    pub fn execute(&self) {
        let p = self.factory.create("pikachu", 5);
        println!("spawned {}", p.name);
    }
}