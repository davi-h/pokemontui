use crate::pokemon::stats::Stats;

pub fn calculate_damage(attacker: &Stats, defender: &Stats) -> u16 {
    attacker.attack.saturating_sub(defender.defense / 2)
}