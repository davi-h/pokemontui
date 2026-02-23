use crate::router::{
    command::Command,
    context::Context,
    error::RouterError,
};

use engine::battle::{engine::BattleEngine, state::BattleState, action::BattleAction};
use engine::factory::pokemon_factory::DefaultPokemonFactory;
use infrastructure::rng::seeded_rng::SeededRng;
use engine::factory::pokemon_factory::PokemonFactory;

use crate::bootstrap::container::build_container;

pub struct BattleCommand;

impl Command for BattleCommand {
    fn name(&self) -> &'static str {
        "battle"
    }

    fn description(&self) -> &'static str {
        "Start a test battle"
    }

    fn execute(&self, _ctx: &mut Context) -> Result<(), RouterError> {
        let mut container = build_container();

        let player;
        let enemy;

        {
            let factory = container
                .resolve::<DefaultPokemonFactory<SeededRng>>()
                .unwrap();

            player = factory.create("pikachu", 5);
            enemy = factory.create("bulbasaur", 5);
        }

        let mut engine = container.resolve_mut::<BattleEngine>().unwrap();
        let mut state = BattleState::new(player, enemy);

        println!("Battle Start! {} vs {}", state.player.name, state.enemy.name);

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

        Ok(())
    }
}