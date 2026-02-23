use contracts::pokedex::PokedexRepository;

pub struct GetPokedex<R: PokedexRepository> {
    repo: R,
}

impl<R: PokedexRepository> GetPokedex<R> {
    pub fn execute(&self) -> Vec<String> {
        self.repo.list()
    }
}