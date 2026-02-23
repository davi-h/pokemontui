pub trait PokedexRepository {
    fn list(&self) -> Vec<String>;
}