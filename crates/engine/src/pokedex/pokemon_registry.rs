use std::process::Command;
use contracts::api::pokemon_data_source::PokemonDataSource;
use contracts::api::PokemonApiData;
use contracts::api::error::ApiError;

/// Representa um Pokémon disponível no jogo
#[derive(Debug, Clone)]
pub struct PokemonEntry {
    pub name: String,
    pub types: Vec<String>,
    pub base: PokemonApiData,
}

/// Registro global de espécies disponíveis
pub struct PokemonRegistry {
    entries: Vec<PokemonEntry>,
}

impl PokemonRegistry {
    /// Cria registry vazio
    pub fn empty() -> Self {
        Self { entries: Vec::new() }
    }

    /// Constrói registry automaticamente
    pub fn load<D: PokemonDataSource>(
        data: &D,
        names: &[String],
    ) -> Result<Self, ApiError> {
        let mut entries = Vec::new();

        for name in names {
            // verifica suporte do pokeget
            if !pokeget_exists(name) {
                continue;
            }

            // busca dados da API
            let api_data = data.fetch(name)?;

            entries.push(PokemonEntry {
                name: api_data.name.clone(),
                types: api_data.types.clone(),
                base: api_data,
            });
        }

        Ok(Self { entries })
    }

    /// Lista todos
    pub fn all(&self) -> &[PokemonEntry] {
        &self.entries
    }

    /// Nome aleatório (para factory)
    pub fn names(&self) -> Vec<String> {
        self.entries.iter().map(|e| e.name.clone()).collect()
    }

    /// Busca específico
    pub fn get(&self, name: &str) -> Option<&PokemonEntry> {
        self.entries.iter().find(|p| p.name == name)
    }
}

/// testa se pokeget suporta
fn pokeget_exists(name: &str) -> bool {
    Command::new("pokeget")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}