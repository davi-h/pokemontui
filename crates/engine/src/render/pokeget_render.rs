// engine/src/render/pokeget_renderer.rs

use std::process::Command;
use std::io;

use domain::pokemon::entity::Pokemon;

/// Responsável por renderizar sprites de Pokémon no terminal
/// utilizando o comando externo `pokeget`.
///
/// Requisitos:
/// - pokeget instalado no sistema
/// - disponível no PATH
pub struct PokegetRenderer;

impl PokegetRenderer {
    /// Cria novo renderer
    pub fn new() -> Self {
        Self
    }

    /// Renderiza sprite baseado no nome do Pokémon
    pub fn render_name(&self, name: &str) -> io::Result<()> {
        let status = Command::new("pokeget")
            .arg(name)
            .status()?;

        if !status.success() {
            eprintln!("Erro ao renderizar sprite de {}", name);
        }

        Ok(())
    }

    /// Renderiza sprite baseado na entidade Pokémon
    pub fn render(&self, pokemon: &Pokemon) -> io::Result<()> {
        if pokemon.shiny {
            self.render_shiny(&pokemon.name)
        } else {
            self.render_name(&pokemon.name)
        }
    }

    /// Renderiza versão shiny
    pub fn render_shiny(&self, name: &str) -> io::Result<()> {
        let status = Command::new("pokeget")
            .arg(name)
            .arg("--shiny")
            .status()?;

        if !status.success() {
            eprintln!("Erro ao renderizar sprite shiny de {}", name);
        }

        Ok(())
    }

    /// Verifica se pokeget está instalado
    pub fn check_available() -> bool {
        Command::new("pokeget")
            .arg("--help")
            .output()
            .is_ok()
    }
}