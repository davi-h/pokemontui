use contracts::rng::Rng;
use rand::{SeedableRng, RngCore};

pub struct SeededRng {
    inner: rand::rngs::StdRng,
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        Self {
            inner: rand::rngs::StdRng::seed_from_u64(seed),
        }
    }
}

impl Rng for SeededRng {
    fn next_u32(&mut self) -> u32 {
        self.inner.next_u32()
    }
}