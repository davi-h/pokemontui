use crate::bootstrap::container::build_container;
use crate::container::factory::build_usecases;

use engine::factory::pokemon_factory::DefaultPokemonFactory;
use infrastructure::rng::seeded_rng::SeededRng;
use infrastructure::api::pokeapi_client::PokeApiClient;

pub struct Application;

impl Application {
    pub fn build() -> Self {
        let mut container = build_container();
        let mut usecases = build_usecases(&mut container);
        usecases.start_battle.execute();
        Self
    }

    pub fn run(self) {
        use engine::factory::pokemon_factory::PokemonFactory;
        use engine::battle::{engine::BattleEngine, state::BattleState, action::BattleAction};

        let mut container = build_container();

        /*
        ========================
        CREATE POKÉMONS
        ========================
        */

        let (player, enemy) = {
            let factory = container
                .resolve_mut::<DefaultPokemonFactory<SeededRng, PokeApiClient>>()
                .expect("Factory not registered");

            let p = factory.create("pikachu", 5).expect("failed to create player");
            let e = factory.create("bulbasaur", 5).expect("failed to create enemy");

            (p, e)
        };

        /*
        ========================
        ENGINE
        ========================
        */

        let engine = container
            .resolve_mut::<BattleEngine>()
            .expect("BattleEngine missing");

        let mut state = BattleState::new(player, enemy);

        println!("Battle Start! {} vs {}", state.player.name, state.enemy.name);

        /*
        ========================
        LOOP
        ========================
        */

        while !state.finished {
            println!("\nTurn {}:", state.turn + 1);
            println!(
                "Player HP: {} | Enemy HP: {}",
                state.player.stats.hp,
                state.enemy.stats.hp
            );

            engine.step(&mut state, BattleAction::Attack);
            println!("Player attacks!");

            if state.enemy.stats.hp == 0 {
                println!("Enemy fainted! You win!");
                break;
            }

            engine.step(&mut state, BattleAction::Attack);
            println!("Enemy attacks!");

            if state.player.stats.hp == 0 {
                println!("You fainted! Game over.");
                break;
            }
        }
    }
}