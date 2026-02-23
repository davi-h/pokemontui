use super::{context::Context, registry::CommandRegistry, error::RouterError};

pub struct Router {
    registry: CommandRegistry,
}

impl Router {
    pub fn new(registry: CommandRegistry) -> Self {
        Self { registry }
    }

    pub fn run(&self, args: Vec<String>) -> Result<(), RouterError> {
        let mut ctx = Context::new(args.clone());

        let cmd_name = args.get(1)
            .ok_or_else(|| RouterError::UnknownCommand("No command provided".into()))?;

        let command = self.registry
            .get(cmd_name)
            .ok_or_else(|| RouterError::UnknownCommand(cmd_name.clone()))?;

        command.execute(&mut ctx)
    }
}