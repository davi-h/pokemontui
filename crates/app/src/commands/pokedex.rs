use engine::pokedex::service::PokedexService;
use infrastructure::{api::pokeapi_client::PokeApiClient, sprites::sprite_loader::SpriteLoader};
use contracts::api::pokemon_data_source::PokemonDataSource;


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
        let names = vec![
            "bulbasaur".into(),
            "charmander".into(),
            "squirtle".into(),
        ];

        let entries = self.service.build_entries(names);
        let loader = SpriteLoader::new("assets/sprites");

        for e in entries {
            println!("==== {} (lvl {}) ====", e.name, e.level);
            loader.fetch(&e.name);
        }
    }
}