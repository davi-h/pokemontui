use engine::spawn::engine::SpawnEngine;
use engine::spawn::entry::SpawnEntry;
use engine::spawn::table::SpawnTable;

#[test]
fn deterministic_spawn_distribution() {
    let table = SpawnTable {
        biome: "grassland".into(),
        entries: vec![
            SpawnEntry {
                species: "pidgey".into(),
                base_weight: 50,
                min_level: 2,
                max_level: 5,
            },
            SpawnEntry {
                species: "pikachu".into(),
                base_weight: 10,
                min_level: 3,
                max_level: 6,
            },
            SpawnEntry {
                species: "eevee".into(),
                base_weight: 5,
                min_level: 4,
                max_level: 7,
            },
        ],
    };

    use engine::spawn::context::{SpawnContext, TimeOfDay, Weather};
    use engine::spawn::registry::SpawnRegistry;

    let mut registry = SpawnRegistry::new();
    registry.register(table.clone());

    let ctx = SpawnContext {
        biome: "grassland".into(),
        time_of_day: TimeOfDay::Day,
        weather: Weather::Clear,
        player_level: 1,
    };

    let mut engine = SpawnEngine::new(42);
    let spawn = engine.spawn(&ctx, &registry).unwrap();

    assert_eq!(spawn.species, "pidgey");
    assert!(spawn.level >= 2 && spawn.level <= 5);
}
