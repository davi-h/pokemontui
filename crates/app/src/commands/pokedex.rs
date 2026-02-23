use engine::pokedex::service::PokedexService;
use adapters::{pokeapi_client::PokeApiClient, sprite_renderer::SpriteRenderer};

pub struct PokedexCommand {
    service: PokedexService,
    api: PokeApiClient,
}

impl PokedexCommand {
    pub fn new() -> Self {
        Self {
            service: PokedexService::new(),
            api: PokeApiClient::new(),
        }
    }

    pub fn run(&mut self) {
        let names = self.api.fetch(
            self.service.page() * 9,
            9
        );

        let entries = self.service.build_entries(names);

        for e in entries {
            println!("==== {} (lvl {}) ====", e.name, e.level);
            SpriteRenderer::show(&e.name);
        }
    }
}