use rand::Rng as _;
use contracts::rng::Rng;

pub struct DefaultRng;

impl Rng for DefaultRng {
    fn range(&mut self, range: std::ops::Range<u32>) -> u32 {
        rand::thread_rng().gen_range(range)
    }
}