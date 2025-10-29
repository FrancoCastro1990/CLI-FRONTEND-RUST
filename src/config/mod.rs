mod architecture;
mod loader;
mod parser;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Re-export public types
pub use architecture::{ArchitectureConfig, ArchitectureStructure};

/// Global configuration for the CLI tool
///
/// Manages default settings, directory locations, and template options.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    default_type: String,
    create_folder: bool,
    enable_hooks: bool,
    templates_dir: PathBuf,
    output_dir: PathBuf,
    architectures_dir: PathBuf,
    default_architecture: String,
}

impl Default for Config {
    fn default() -> Self {
        // Try multiple locations for templates directory
        let templates_dir = Self::find_templates_directory();
        let architectures_dir = Self::find_architectures_directory();

        Self {
            default_type: "component".to_string(),
            create_folder: true,
            enable_hooks: true,
            templates_dir,
            output_dir: PathBuf::from("."),
            architectures_dir,
            default_architecture: "screaming-architecture".to_string(),
        }
    }
}

impl Config {
    // Getters for private fields
    pub fn default_type(&self) -> &str {
        &self.default_type
    }

    pub fn create_folder(&self) -> bool {
        self.create_folder
    }

    #[allow(dead_code)]
    pub fn enable_hooks(&self) -> bool {
        self.enable_hooks
    }

    pub fn templates_dir(&self) -> &PathBuf {
        &self.templates_dir
    }

    pub fn output_dir(&self) -> &PathBuf {
        &self.output_dir
    }

    pub fn architectures_dir(&self) -> &PathBuf {
        &self.architectures_dir
    }

    pub fn default_architecture(&self) -> &str {
        &self.default_architecture
    }

    /// Load architecture configuration from JSON file
    pub async fn load_architecture(&self, architecture_name: &str) -> Result<ArchitectureConfig> {
        ArchitectureConfig::load_from_file(&self.architectures_dir, architecture_name).await
    }

    /// List all available architectures
    #[allow(dead_code)]
    pub fn list_architectures(&self) -> Result<Vec<String>> {
        ArchitectureConfig::list_in_directory(&self.architectures_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.default_type(), "component");
        assert!(config.create_folder());
        assert!(config.enable_hooks());
        assert_eq!(config.default_architecture(), "screaming-architecture");
    }

    #[test]
    fn test_config_getters() {
        let config = Config::default();
        // Verify all getters return the expected types and values
        let _dt: &str = config.default_type();
        let _cf: bool = config.create_folder();
        let _eh: bool = config.enable_hooks();
        let _td: &PathBuf = config.templates_dir();
        let _od: &PathBuf = config.output_dir();
        let _ad: &PathBuf = config.architectures_dir();
        let _da: &str = config.default_architecture();
    }

    #[test]
    fn test_find_templates_directory() {
        let templates_dir = Config::find_templates_directory();
        // Should return a PathBuf (existence not guaranteed in test environment)
        assert!(templates_dir.to_str().is_some());
    }
}
