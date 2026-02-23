pub trait PokemonApi {
    fn fetch(&self, name: &str) -> String;
}