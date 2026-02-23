use domain::pokemon::stats::Stats;

pub fn calculate_damage(attacker: &Stats, defender: &Stats) -> u16 {
    let base = attacker.attack.saturating_sub(defender.defense / 2);
    std::cmp::max(base, 1)
}