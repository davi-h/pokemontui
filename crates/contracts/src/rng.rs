use std::ops::Range;

pub trait Rng {
    /// Número bruto
    fn next_u32(&mut self) -> u32;

    /// Range inteiro
    fn range(&mut self, range: Range<usize>) -> usize {
        let size = range.end - range.start;
        (self.next_u32() as usize % size) + range.start
    }

    /// Float 0.0 — 1.0
    fn float(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    }

    /// Probabilidade simples
    fn chance(&mut self, probability: f32) -> bool {
        self.float() <= probability
    }

    /// Escolher item de slice
    fn choose<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        let index = self.range(0..items.len());
        &items[index]
    }

    /// Escolha ponderada
    fn weighted<'a, T>(&mut self, items: &'a [(T, u32)]) -> &'a T {
        let total: u32 = items.iter().map(|(_, w)| *w).sum();
        let mut roll = self.next_u32() % total;

        for (item, weight) in items {
            if roll < *weight {
                return item;
            }
            roll -= *weight;
        }

        &items[0].0
    }
}