use std::collections::HashSet;

use contracts::api::pokemon_data_source::PokemonDataSource;
use contracts::api::error::ApiError;

use infrastructure::cache::file_cache::FileCache;
use infrastructure::sprites::sprite_loader::SpriteLoader;

/// Registro global de espécies válidas.
///
/// Responsabilidades:
/// - armazenar nomes suportados
/// - garantir compatibilidade com renderer
/// - evitar spawn inválido
#[derive(Clone)]
pub struct SpeciesRegistry {
    species: HashSet<String>,
}

impl SpeciesRegistry {
    /// Cria registry vazio
    pub fn empty() -> Self {
        Self {
            species: HashSet::new(),
        }
    }

    /// Verifica se espécie existe
    pub fn contains(&self, name: &str) -> bool {
        self.species.contains(name)
    }

    /// Lista todas espécies
    pub fn all(&self) -> impl Iterator<Item = &String> {
        self.species.iter()
    }

    /// Quantidade total
    pub fn len(&self) -> usize {
        self.species.len()
    }

    /// Carrega registry completo
    ///
    /// Estratégia:
    /// 1 — tenta cache
    /// 2 — se não existir → gera
    /// 3 — filtra sprites válidos
    /// 4 — salva cache
    pub fn load<D>(
        data: &D,
        sprites: &SpriteLoader,
        cache: &FileCache,
    ) -> Result<Self, ApiError>
    where
        D: PokemonDataSource,
    {
        // ---------- cache ----------
        if let Some(cached) = cache.get("species.txt") {
            return Ok(Self {
                species: cached
                    .lines()
                    .map(|s| s.to_string())
                    .collect::<HashSet<String>>(),
            });
        }

        // ---------- fallback build ----------
        let mut set = HashSet::new();

        // lista inicial mínima
        // (depois trocamos por endpoint real)
        let seed_list = [
            "bulbasaur","ivysaur","venusaur",
            "charmander","charmeleon","charizard",
            "squirtle","wartortle","blastoise",
            "pikachu","raichu",
        ];

        for name in seed_list {
            // sprite precisa existir
            if !sprites.exists(name) {
                continue;
            }

            // API precisa responder
            if data.fetch(name).is_ok() {
                set.insert(name.to_string());
            }
        }

        // ---------- salvar cache ----------
        let serialized = set.iter().cloned().collect::<Vec<_>>().join("\n");
        cache.set("species.txt", &serialized);

        Ok(Self { species: set })
    }
}