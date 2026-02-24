use std::collections::HashSet;
use std::sync::Arc;

#[derive(Clone)]
pub struct SpeciesRegistry {
    list: Arc<[String]>,
    index: HashSet<String>,
}

impl SpeciesRegistry {
    pub fn new(list: impl IntoIterator<Item = String>) -> Self {
        let vec: Vec<String> = list.into_iter().collect();
        let index = vec.iter().cloned().collect();

        Self {
            list: vec.into(),
            index,
        }
    }

    #[inline]
    pub fn contains(&self, name: &str) -> bool {
        self.index.contains(name)
    }

    #[inline]
    pub fn all(&self) -> impl Iterator<Item = &String> {
        self.list.iter()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.list.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}