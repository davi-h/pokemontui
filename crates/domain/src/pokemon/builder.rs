use super::{entity::Pokemon, stats::Stats};

/// Builder responsável pela construção segura e flexível de um Pokémon.
/// Permite configurar propriedades passo a passo antes da criação final.
pub struct PokemonBuilder {
    name: String,
    level: u8,
    stats: Stats,
    shiny: bool,
}

impl PokemonBuilder {
    /// Cria um builder base com valores padrão seguros.
    pub fn new(name: String) -> Self {
        Self {
            name,
            level: 1,
            stats: Stats::zero(),
            shiny: false,
        }
    }

    /// Define o nível do Pokémon.
    pub fn level(mut self, level: u8) -> Self {
        self.level = level;
        self
    }

    /// Define os stats manualmente.
    pub fn stats(mut self, stats: Stats) -> Self {
        self.stats = stats;
        self
    }

    /// Define se é shiny.
    pub fn shiny(mut self, shiny: bool) -> Self {
        self.shiny = shiny;
        self
    }

    /// Finaliza a construção e gera o Pokémon.
    pub fn build(self) -> Pokemon {
        Pokemon {
            name: self.name,
            level: self.level,
            stats: self.stats,
            shiny: self.shiny,
        }
    }
}