use super::models::*;
use domain::pokemon::{entity::Pokemon, builder::PokemonBuilder, stats::Stats};

pub fn parse_pokemon(json: &str, level: u8) -> anyhow::Result<Pokemon> {
    let api: ApiPokemon = serde_json::from_str(json)?;

    let mut hp = 10;
    let mut attack = 10;
    let mut defense = 10;
    let mut speed = 10;

    for s in api.stats {
        match s.stat.name.as_str() {
            "hp" => hp = s.base_stat,
            "attack" => attack = s.base_stat,
            "defense" => defense = s.base_stat,
            "speed" => speed = s.base_stat,
            _ => {}
        }
    }

    let stats = Stats {
        hp,
        attack,
        defense,
        speed,
    };

    Ok(
        PokemonBuilder::new(api.name)
            .level(level)
            .stats(stats)
            .build()
    )
}