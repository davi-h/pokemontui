use contracts::rng::Rng;

pub struct SpawnPool {
    entries: Vec<(String, f32)>
}

impl SpawnPool {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add(&mut self, name: impl Into<String>, weight: f32) {
        self.entries.push((name.into(), weight));
    }

    pub fn weight(&mut self, name: &str, multiplier: f32) {
        for (n, w) in &mut self.entries {
            if n == name {
                *w *= multiplier;
            }
        }
    }

    pub fn choose<R: Rng>(&self, rng: &mut R) -> String {
        let total: f32 = self.entries.iter().map(|e| e.1).sum();

        let mut roll = rng.f32() * total;

        for (name, weight) in &self.entries {
            if roll < *weight {
                return name.clone();
            }
            roll -= weight;
        }

        self.entries.last().unwrap().0.clone()
    }
}