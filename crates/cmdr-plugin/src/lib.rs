//! Plugin system for cmdr
//! 
//! This crate provides a plugin system for extending cmdr's functionality
//! through dynamically loaded libraries.

use libloading::{Library, Symbol};
use std::any::Any;
use std::path::Path;

/// Plugin trait that all cmdr plugins must implement
pub trait Plugin: Send + Sync {
    /// Get the name of the plugin
    fn name(&self) -> &str;
    
    /// Get the version of the plugin
    fn version(&self) -> &str;
    
    /// Initialize the plugin
    fn init(&mut self) -> anyhow::Result<()>;
    
    /// Clean up the plugin
    fn cleanup(&mut self) -> anyhow::Result<()>;
    
    /// Get plugin-specific data
    fn as_any(&self) -> &dyn Any;
}

/// Plugin manager for loading and managing plugins
pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    libraries: Vec<Library>,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            libraries: Vec::new(),
        }
    }
    
    /// Load a plugin from a dynamic library
    pub fn load_plugin<P: AsRef<Path>>(&mut self, path: P) -> anyhow::Result<()> {
        let library = unsafe { Library::new(path.as_ref())? };
        
        // Load the plugin creation function
        let create_plugin: Symbol<fn() -> Box<dyn Plugin>> = unsafe {
            library.get(b"create_plugin")?
        };
        
        let mut plugin = create_plugin();
        plugin.init()?;
        
        self.plugins.push(plugin);
        self.libraries.push(library);
        
        Ok(())
    }
    
    /// Get all loaded plugins
    pub fn plugins(&self) -> &[Box<dyn Plugin>] {
        &self.plugins
    }
    
    /// Find a plugin by name
    pub fn find_plugin(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.iter().find(|p| p.name() == name).map(|p| p.as_ref())
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        // Clean up all plugins
        for plugin in &mut self.plugins {
            let _ = plugin.cleanup();
        }
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        assert_eq!(manager.plugins().len(), 0);
    }
} 