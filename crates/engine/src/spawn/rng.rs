
#[derive(Clone)]
pub struct SeededRng {
    state: u64,
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u32(&mut self) -> u32 {
        // Xorshift64*
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        ((x.wrapping_mul(2685821657736338717)) >> 32) as u32
    }

    pub fn range(&mut self, min: u32, max: u32) -> u32 {
        min + (self.next_u32() % (max - min))
    }
}
