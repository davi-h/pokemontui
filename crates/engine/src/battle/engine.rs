impl Default for BattleEngine {
    fn default() -> Self {
        Self::new()
    }
}
use super::{state::BattleState, action::BattleAction};
use crate::turn::manager::TurnManager;
use super::resolver::BattleResolver;

pub struct BattleEngine {
    turn_manager: TurnManager,
}

impl BattleEngine {
    pub fn new() -> Self {
        Self {
            turn_manager: TurnManager::new(),
        }
    }

    pub fn step(
        &mut self,
        state: &mut BattleState,
        player_action: BattleAction,
    ) {
        if state.finished {
            return;
        }

        self.turn_manager.next_turn();

        BattleResolver::resolve(state, player_action);

        state.turn += 1;
    }
}