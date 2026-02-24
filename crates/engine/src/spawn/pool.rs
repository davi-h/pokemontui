use contracts::rng::Rng;

pub struct SpawnPool {
    entries: Vec<(String, f32)>
}

impl SpawnPool {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add(&mut self, name: impl Into<String>, weight: f32) {
        if weight > 0.0 {
            self.entries.push((name.into(), weight));
        }
    }

    pub fn weight(&mut self, name: &str, multiplier: f32) {
        for (n, w) in &mut self.entries {
            if n == name {
                *w *= multiplier;
            }
        }
    }

    pub fn choose<R: Rng>(&self, rng: &mut R) -> String {
        assert!(
            !self.entries.is_empty(),
            "SpawnPool::choose chamado com pool vazia"
        );

        let total: f32 = self.entries.iter().map(|e| e.1).sum();

        if total <= 0.0 {
            return self.entries.last().unwrap().0.clone();
        }

        // converte RNG inteiro → float determinístico
        let roll = {
            let r = rng.u32(0, u32::MAX);
            (r as f32 / u32::MAX as f32) * total
        };

        let mut acc = 0.0;

        for (name, weight) in &self.entries {
            acc += *weight;
            if roll < acc {
                return name.clone();
            }
        }

        self.entries.last().unwrap().0.clone()
    }
}