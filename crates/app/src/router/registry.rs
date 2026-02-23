use std::collections::HashMap;
use super::command::Command;

pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register<C: Command + 'static>(&mut self, command: C) {
        self.commands.insert(command.name().into(), Box::new(command));
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn Command>> {
        self.commands.get(name)
    }

    pub fn list(&self) -> Vec<&String> {
        self.commands.keys().collect()
    }
}