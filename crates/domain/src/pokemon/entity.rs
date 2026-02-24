use super::stats::Stats;

#[derive(Clone, Debug)]
pub struct Pokemon {
    pub name: String,
    pub level: u8,
    pub stats: Stats,
    pub shiny: bool,
}

impl Pokemon {
    /// Construtor direto (usado raramente — builder é preferido)
    pub fn new(name: String, level: u8, stats: Stats) -> Self {
        Self {
            name,
            level,
            stats,
            shiny: false,
        }
    }

    /// Define se é shiny
    pub fn set_shiny(&mut self, shiny: bool) {
        self.shiny = shiny;
    }

    /// Versão builder-style
    pub fn with_shiny(mut self, shiny: bool) -> Self {
        self.shiny = shiny;
        self
    }
}