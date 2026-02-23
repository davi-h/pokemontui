impl Default for PluginRegistry {
	fn default() -> Self {
		Self::new()
	}
}
pub mod commands;
pub mod queries;
pub mod services;

/// Plugins must implement this trait to be registered with the application.
pub trait Plugin {
	fn name(&self) -> &'static str;
	fn initialize(&self);
}

/// Application plugin registry (simple example, can be extended for dynamic loading).
pub struct PluginRegistry {
	plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistry {
	pub fn new() -> Self {
		Self { plugins: Vec::new() }
	}

	pub fn register(&mut self, plugin: Box<dyn Plugin>) {
		self.plugins.push(plugin);
	}

	pub fn initialize_all(&self) {
		for plugin in &self.plugins {
			plugin.initialize();
		}
	}
}