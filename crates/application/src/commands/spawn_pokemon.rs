use contracts::spawn::SpawnService;

pub struct SpawnPokemon<S: SpawnService> {
    service: S,
}

impl<S: SpawnService> SpawnPokemon<S> {
    pub fn new(service: S) -> Self {
        Self { service }
    }

    pub fn execute(&self) {
        self.service.spawn();
    }
}