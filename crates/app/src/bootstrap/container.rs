use crate::container::registry::ServiceRegistry;

use infrastructure::{
    api::pokeapi_client::PokeApiClient,
    cache::file_cache::FileCache,
    storage::save_repository::SaveRepository,
    spawn::spawn_service::DefaultSpawnService,
    sprites::sprite_loader::SpriteLoader,
};

use engine::battle::engine::BattleEngine;
use infrastructure::rng::seeded_rng::SeededRng;
use engine::factory::pokemon_factory::DefaultPokemonFactory;


pub fn build_container() -> ServiceRegistry {
    let mut c = ServiceRegistry::new();
    let rng = SeededRng::new(42);
    let factory = DefaultPokemonFactory::new(rng);
    c.register(factory);
    // infra
    c.register(PokeApiClient::new());
    c.register(FileCache::new("assets/cache"));
    c.register(SaveRepository::new("assets/saves/save.json"));
    c.register(DefaultSpawnService);
    c.register(SpriteLoader);

    // engine
    c.register(BattleEngine::new());

    c
}