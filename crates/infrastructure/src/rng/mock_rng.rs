use contracts::rng::{GameRng, Rng};

pub struct MockRng {
    pub value: u32,
}

impl Rng for MockRng {
    fn next_u32(&mut self) -> u32 {
        self.value
    }
}

impl GameRng for MockRng {
    fn range_u32(&mut self, _range: std::ops::Range<u32>) -> u32 {
        self.value
    }
}
