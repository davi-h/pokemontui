use contracts::rng::Rng;
use rand::{SeedableRng, RngCore};
use std::ops::Range;

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
        // Xorshift64*
//        let mut x = self.state;
//        x ^= x >> 12;
//        x ^= x << 25;
//        x ^= x >> 27;
//        self.state = x;
//        ((x.wrapping_mul(2685821657736338717)) >> 32) as u32
    }

    fn range(&mut self, range: Range<usize>) -> usize {
        let size = range.end - range.start;
        (self.next_u32() as usize % size) + range.start
    }
}