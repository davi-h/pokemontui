#[derive(Clone)]
pub struct WeightedEntry<T> {
    pub item: T,
    pub weight: u32,
}

impl<T> WeightedEntry<T> {
    pub fn new(item: T, weight: u32) -> Self {
        Self { item, weight }
    }
}