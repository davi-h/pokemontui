use crate::bootstrap::container::build_container;
use crate::container::factory::build_usecases;

pub struct Application;

impl Application {
    pub fn build() -> Self {
        let container = build_container();
        let mut container = build_container();
        let mut usecases = build_usecases(&mut container);
        usecases.start_battle.execute();

        Self
    }

    pub fn run(self) {
        use engine::factory::pokemon_factory::PokemonFactory;
        use engine::battle::{engine::BattleEngine, state::BattleState, action::BattleAction};
        use crate::bootstrap::container::build_container;
        use crate::container::registry::ServiceRegistry;

        // Setup DI container
        let mut container = build_container();
        // Resolve factory, create pokémon, depois engine mutável
        let player;
        let enemy;
        {
            let factory = container.resolve::<engine::factory::pokemon_factory::DefaultPokemonFactory<infrastructure::rng::seeded_rng::SeededRng>>().unwrap();
            player = factory.create("pikachu", 5);
            enemy = factory.create("bulbasaur", 5);
        }

        let mut engine = container.resolve_mut::<BattleEngine>().unwrap();
        let mut state = BattleState::new(player, enemy);

        println!("Battle Start! {} vs {}", state.player.name, state.enemy.name);

        // Simple loop: player always attacks, enemy always attacks
        while !state.finished {
            println!("\nTurn {}:", state.turn + 1);
            println!("Player HP: {} | Enemy HP: {}", state.player.stats.hp, state.enemy.stats.hp);

            // Player attacks
            engine.step(&mut state, BattleAction::Attack);
            println!("Player attacks!");
            if state.enemy.stats.hp == 0 {
                println!("Enemy fainted! You win!");
                break;
            }

            // Enemy attacks
            engine.step(&mut state, BattleAction::Attack);
            println!("Enemy attacks!");
            if state.player.stats.hp == 0 {
                println!("You fainted! Game over.");
                break;
            }
        }
    }
}