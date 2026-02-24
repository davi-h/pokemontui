use std::ops::Range;

/// Contrato enxuto e universal para geração de números pseudoaleatórios.
///
/// Esse trait é ideal para uso em Application/Domain/Engine, sem acoplamento
/// à crate `rand`.
pub trait GameRng {
    fn range_u32(&mut self, range: Range<u32>) -> u32;

    fn chance_percent(&mut self, percent: u8) -> bool {
        let clamped = percent.min(100) as u32;
        self.range_u32(0..100) < clamped
    }
}

pub trait Rng {
    /// Número bruto
    fn next_u32(&mut self) -> u32;

    /// Range inteiro (usize)
    fn range(&mut self, range: Range<usize>) -> usize {
        let size = range.end - range.start;
        (self.next_u32() as usize % size) + range.start
    }

    /// Range inteiro u32
    fn u32(&mut self, min: u32, max: u32) -> u32 {
        min + (self.next_u32() % (max - min))
    }

    /// Float 0.0 — 1.0
    fn float(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    }

    /// Alias compatibilidade
    fn f32(&mut self) -> f32 {
        self.float()
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

<<<<<<< HEAD
/// Compatibilidade: qualquer implementação legacy de `Rng` passa a atender
/// também o contrato `GameRng`.
impl<T: Rng + ?Sized> GameRng for T {
    fn range_u32(&mut self, range: Range<u32>) -> u32 {
        self.u32(range.start, range.end)
    }
}
=======

/// Trait de RNG abstrato usado pelo domínio.
/// 
/// Motivo:
/// - permitir mock em testes
/// - desacoplar engine de rand crate
/// - permitir RNG determinístico
pub trait GameRng {
    fn range(&mut self, range: std::ops::Range<u32>) -> u32;
}
>>>>>>> 694a416 (v0.0.4)
