use contracts::rng::GameRng;
use engine::probability::{
    CritOutcome, HitOutcome, ProbabilitySystem, StatusEffect, StatusOutcome,
};

struct FixedRng {
    value: u32,
}

impl GameRng for FixedRng {
    fn range_u32(&mut self, _range: std::ops::Range<u32>) -> u32 {
        self.value
    }
}

#[test]
fn hit_roll_respects_percentage() {
    let mut p = ProbabilitySystem::new(FixedRng { value: 10 });
    assert_eq!(p.roll_hit(20), HitOutcome::Hit);
    assert_eq!(p.roll_hit(5), HitOutcome::Miss);
}

#[test]
fn crit_roll_respects_percentage() {
    let mut p = ProbabilitySystem::new(FixedRng { value: 2 });
    assert_eq!(p.roll_crit(3), CritOutcome::Critical);
    assert_eq!(p.roll_crit(2), CritOutcome::Normal);
}

#[test]
fn status_roll_applies_effect() {
    let mut p = ProbabilitySystem::new(FixedRng { value: 15 });
    assert_eq!(
        p.roll_status(StatusEffect::Burn, 30),
        StatusOutcome::Applied(StatusEffect::Burn)
    );
    assert_eq!(p.roll_status(StatusEffect::Poison, 10), StatusOutcome::NotApplied);
}
