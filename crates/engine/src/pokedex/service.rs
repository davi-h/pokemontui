use super::{model::PokedexEntry, paginator::Paginator};

pub struct PokedexService {
    paginator: Paginator,
}

impl PokedexService {
    pub fn new() -> Self {
        Self {
            paginator: Paginator::new(9),
        }
    }

    pub fn page(&self) -> usize {
        self.paginator.page()
    }

    pub fn next(&mut self) {
        self.paginator.next();
    }

    pub fn prev(&mut self) {
        self.paginator.prev();
    }

    pub fn build_entries(&self, names: Vec<String>) -> Vec<PokedexEntry> {
        names.into_iter().enumerate().map(|(i, n)| {
            PokedexEntry {
                id: i as u32,
                name: n,
                level: (5 + i as u8),
            }
        }).collect()
    }
}