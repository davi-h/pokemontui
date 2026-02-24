use engine::factory::pokemon_factory::PokemonFactory;

pub struct SpawnPokemon<F: PokemonFactory> {
    factory: F,
}

impl<F: PokemonFactory> SpawnPokemon<F> {
    pub fn execute(&mut self) {
        let p = self.factory.create("pikachu", 5);
        match p {
        Ok(p) => println!("{}", p.name),
        Err(e) => println!("spawn failed: {:?}", e),
        }
    }
}