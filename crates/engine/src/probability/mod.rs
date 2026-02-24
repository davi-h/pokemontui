use contracts::rng::GameRng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitOutcome {
    Hit,
    Miss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CritOutcome {
    Normal,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusEffect {
    Burn,
    Poison,
    Paralysis,
    Sleep,
    Freeze,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusOutcome {
    Applied(StatusEffect),
    NotApplied,
}

pub struct ProbabilitySystem<R: GameRng> {
    rng: R,
}

impl<R: GameRng> ProbabilitySystem<R> {
    pub fn new(rng: R) -> Self {
        Self { rng }
    }

    pub fn roll_hit(&mut self, hit_chance_percent: u8) -> HitOutcome {
        if self.rng.chance_percent(hit_chance_percent) {
            HitOutcome::Hit
        } else {
            HitOutcome::Miss
        }
    }

    pub fn roll_crit(&mut self, crit_chance_percent: u8) -> CritOutcome {
        if self.rng.chance_percent(crit_chance_percent) {
            CritOutcome::Critical
        } else {
            CritOutcome::Normal
        }
    }

    pub fn roll_status(
        &mut self,
        effect: StatusEffect,
        apply_chance_percent: u8,
    ) -> StatusOutcome {
        if self.rng.chance_percent(apply_chance_percent) {
            StatusOutcome::Applied(effect)
        } else {
            StatusOutcome::NotApplied
        }
    }

    pub fn rng_mut(&mut self) -> &mut R {
        &mut self.rng
    }
}
