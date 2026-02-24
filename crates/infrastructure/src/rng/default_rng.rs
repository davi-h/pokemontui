use contracts::rng::{GameRng, Rng};
use rand::RngCore;

pub struct DefaultRng;

impl Rng for DefaultRng {
    fn next_u32(&mut self) -> u32 {
        rand::thread_rng().next_u32()
    }
}

impl GameRng for DefaultRng {
    fn range_u32(&mut self, range: std::ops::Range<u32>) -> u32 {
        self.u32(range.start, range.end)
    }
}
