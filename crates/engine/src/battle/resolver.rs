use super::{state::BattleState, action::BattleAction};
use crate::stats::calculator::calculate_damage;

pub struct BattleResolver;

impl BattleResolver {
    pub fn resolve(state: &mut BattleState, action: BattleAction) {
        match action {
            BattleAction::Attack => {
                let dmg = calculate_damage(
                    &state.player.stats,
                    &state.enemy.stats,
                );

                state.enemy.stats.hp =
                    state.enemy.stats.hp.saturating_sub(dmg);

                if state.enemy.stats.hp == 0 {
                    state.finished = true;
                }
            }

            BattleAction::Run => {
                state.finished = true;
            }

            BattleAction::Defend => {}
        }
    }
}