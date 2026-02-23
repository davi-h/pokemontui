use engine::battle::{
    engine::BattleEngine,
    state::BattleState,
    action::BattleAction,
};

use domain::pokemon::{entity::Pokemon, stats::Stats};

pub struct StartBattle<'a> {
    engine: &'a mut BattleEngine,
}

impl<'a> StartBattle<'a> {
    pub fn new(engine: &'a mut BattleEngine) -> Self {
        Self { engine }
    }

    pub fn execute(&mut self) {
        let player = Pokemon {
            name: "pikachu".into(),
            level: 5,
            stats: Stats {
                hp: 35,
                attack: 55,
                defense: 40,
                speed: 90,
            },
        };

        let enemy = player.clone();

        let mut state = BattleState::new(player, enemy);

        self.engine.step(&mut state, BattleAction::Attack);
    }
}