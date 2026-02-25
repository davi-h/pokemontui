use contracts::rng::Rng;
use crate::spawn::table::SpawnEntry;

/// Distribuição ponderada determinística.
///
/// Garantias:
/// - sem overflow silencioso
/// - ignora entradas com peso 0
/// - deterministicamente reproduzível via RNG injetado
pub struct WeightedDistribution;

impl WeightedDistribution {
    pub fn pick<'a, R: Rng>(
        entries: &'a [SpawnEntry],
        rng: &mut R,
    ) -> Option<&'a SpawnEntry> {
        // soma segura ignorando pesos zero
        let total: u32 = entries
            .iter()
            .map(|e| e.base_weight)
            .filter(|w| *w > 0)
            .sum();

        if total == 0 {
            return None;
        }

        // sorteio uniforme no range total
        let roll = rng.u32(0, total);

        let mut acc: u32 = 0;

        for entry in entries {
            if entry.base_weight == 0 {
                continue;
            }

            acc = acc.saturating_add(entry.base_weight);

            if roll < acc {
                return Some(entry);
            }
        }

        // fallback matematicamente impossível
        debug_assert!(false, "WeightedDistribution failed to pick entry");
        None
    }
}