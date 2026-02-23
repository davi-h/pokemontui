use contracts::rng::Rng;
use std::cell::RefCell;
use rand::{SeedableRng, Rng as _, rngs::StdRng};

pub struct SeededRng {
    rng: RefCell<StdRng>,
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: RefCell::new(StdRng::seed_from_u64(seed)),
        }
    }
}

impl Rng for SeededRng {
    fn u32(&self, min: u32, max: u32) -> u32 {
        self.rng.borrow_mut().gen_range(min..=max)
    }
}