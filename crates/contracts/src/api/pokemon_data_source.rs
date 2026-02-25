use crate::api::error::ApiError;

#[derive(Debug, Clone)]
pub struct PokemonApiData {
    pub name: String,
    pub base_stats: BaseStats,
    pub types: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BaseStats {
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub special_attack: u16,
    pub special_defense: u16,
    pub speed: u16,
}

/// Fonte de dados de Pokémon.
///
/// Contrato centralizado para qualquer backend:
/// - PokéAPI
/// - cache local
/// - mock
/// - dataset offline
///
/// Regras:
/// - Implementações NÃO devem aplicar regra de jogo
/// - Apenas transporte + parsing
pub trait PokemonDataSource {
    /// Busca dados de uma espécie
    fn fetch(&self, name: &str) -> Result<PokemonApiData, ApiError>;

    /// Busca múltiplos Pokémons (batch opcional)
    ///
    /// Default fallback: chama fetch individualmente.
    fn fetch_many(&self, names: &[String]) -> Result<Vec<PokemonApiData>, ApiError> {
        let mut out = Vec::with_capacity(names.len());

        for n in names {
            out.push(self.fetch(n)?);
        }

        Ok(out)
    }

    /// Verifica se espécie existe na fonte.
    ///
    /// Default:
    /// tenta fetch e ignora resultado.
    fn exists(&self, name: &str) -> bool {
        self.fetch(name).is_ok()
    }
}