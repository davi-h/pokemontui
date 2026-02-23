use crate::router::{command::Command, context::Context, error::RouterError};

pub struct SpawnCommand;

impl Command for SpawnCommand {
    fn name(&self) -> &'static str {
        "spawn"
    }

    fn description(&self) -> &'static str {
        "Spawns a wild pokemon"
    }

    fn execute(&self, _ctx: &mut Context) -> Result<(), RouterError> {
        println!("A wild Pokémon appeared!");
        Ok(())
    }
}