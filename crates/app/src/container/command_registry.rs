use crate::router::registry::CommandRegistry;
use crate::commands::spawn::SpawnCommand;
use crate::commands::battle::BattleCommand;



pub fn build_registry() -> CommandRegistry {
    let mut registry = CommandRegistry::new();

    registry.register(SpawnCommand);
    registry.register(BattleCommand);

    registry
}