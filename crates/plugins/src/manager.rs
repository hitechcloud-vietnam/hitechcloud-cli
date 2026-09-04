//! Plugin manager

use crate::plugin::Plugin;

/// Plugin manager error
#[derive(Debug, thiserror::Error)]
pub enum PluginManagerError {
    #[error("Plugin not found: {0}")]
    NotFound(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Plugin manager for loading and managing plugins
pub struct PluginManager {
    plugins: Vec<Plugin>,
}

impl PluginManager {
    /// Create a new PluginManager
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// Register a plugin
    pub fn register(&mut self, plugin: Plugin) {
        self.plugins.push(plugin);
    }

    /// Get all registered plugins
    pub fn list(&self) -> &[Plugin] {
        &self.plugins
    }

    /// Find a plugin by name
    pub fn find(&self, name: &str) -> Option<&Plugin> {
        self.plugins.iter().find(|p| p.name == name)
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}
