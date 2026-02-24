use rand::Rng;
use contracts::rng::GameRng;

pub struct DefaultRng;

impl GameRng for DefaultRng {
    fn range(&mut self, range: std::ops::Range<u32>) -> u32 {
        rand::thread_rng().gen_range(range)
    }
}