use contracts::rng::Rng;

pub struct MockRng {
    values: Vec<u32>,
    index: usize,
}

impl MockRng {
    pub fn new(values: Vec<u32>) -> Self {
        Self { values, index: 0 }
    }
}

impl Rng for MockRng {
    fn next_u32(&mut self) -> u32 {
        let v = self.values[self.index];
        self.index = (self.index + 1) % self.values.len();
        v
    }
}