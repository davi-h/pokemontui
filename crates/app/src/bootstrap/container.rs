use crate::container::registry::ServiceRegistry;

use infrastructure::{
    api::pokeapi_client::PokeApiClient,
    cache::file_cache::FileCache,
    storage::save_repository::SaveRepository,
    spawn::spawn_service::DefaultSpawnService,
    rng::seeded_rng::SeededRng,
    sprites::sprite_gateway_impl::SpriteGatewayImpl,
};

use application::pokedex::species_registry_builder::SpeciesRegistryBuilder;

use engine::{
    battle::engine::BattleEngine,
    factory::pokemon_factory::DefaultPokemonFactory,
};

pub async fn build_container() -> ServiceRegistry {
    let mut c = ServiceRegistry::new();

    /*
    =========================
    CORE
    =========================
    */

    let rng = SeededRng::new(42);
    let api = PokeApiClient::new();

    let sprite_gateway =
        SpriteGatewayImpl::new("assets/cache/sprites".into());

    let cache = FileCache::new("assets/cache");
    let save_repo = SaveRepository::new("assets/saves/save.json");

    /*
    =========================
    REGISTRY
    =========================
    */

    let registry = SpeciesRegistryBuilder::build(
        &api,
        &sprite_gateway,
    )
    .await
    .expect("failed to load species");

    let species_pool: Vec<String> =
        registry.all().cloned().collect();

    /*
    =========================
    FACTORY
    =========================
    */

    let pokemon_factory =
        DefaultPokemonFactory::new(
            rng,
            api,
            species_pool,
        );

    /*
    =========================
    ENGINES
    =========================
    */

    let battle_engine = BattleEngine::new();

    /*
    =========================
    SERVICES
    =========================
    */

    let spawn_service = DefaultSpawnService {};

    /*
    =========================
    REGISTRO
    =========================
    */

    c.register(cache);
    c.register(save_repo);
    c.register(sprite_gateway);
    c.register(registry);
    c.register(pokemon_factory);
    c.register(battle_engine);
    c.register(spawn_service);

    c
}