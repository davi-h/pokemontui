pub trait Rng {
    fn u32(&self, min: u32, max: u32) -> u32;
}