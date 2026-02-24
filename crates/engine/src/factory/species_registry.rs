use std::collections::HashSet;

/// Registro global de espécies válidas.
///
/// Responsabilidades:
/// - armazenar nomes suportados
/// - validar espécie
#[derive(Clone)]
pub struct SpeciesRegistry {
    species: HashSet<String>,
}

impl SpeciesRegistry {
    pub fn new(list: impl IntoIterator<Item = String>) -> Self {
        Self {
            species: list.into_iter().collect(),
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.species.contains(name)
    }

    pub fn all(&self) -> impl Iterator<Item = &String> {
        self.species.iter()
    }

    pub fn len(&self) -> usize {
        self.species.len()
    }
}