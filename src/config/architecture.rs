use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;

/// Configuration for a feature architecture pattern
///
/// Defines the structure, benefits, and limitations of an architectural approach.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArchitectureConfig {
    pub name: String,
    pub description: String,
    pub benefits: Vec<String>,
    pub limitations: Vec<String>,
    pub structure: Vec<ArchitectureStructure>,
}

/// A single component of an architecture structure
///
/// Describes where and how to generate files for a specific part of the architecture.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArchitectureStructure {
    pub path: String,
    pub template: String,
    pub filename_pattern: String,
    pub description: String,
}

impl ArchitectureConfig {
    /// Parse architecture JSON content
    pub fn parse_json(content: &str) -> Result<Self> {
        serde_json::from_str(content).context("Failed to parse architecture JSON")
    }

    /// Load architecture configuration from JSON file
    pub async fn load_from_file(architectures_dir: &Path, architecture_name: &str) -> Result<Self> {
        let filename = if architecture_name == "default" {
            "default.json".to_string()
        } else {
            format!("{}.json", architecture_name)
        };

        let architecture_path = architectures_dir.join(&filename);

        if !architecture_path.exists() {
            // Try to load default architecture if requested one doesn't exist
            let default_path = architectures_dir.join("default.json");
            if default_path.exists() {
                let content = fs::read_to_string(&default_path).await.with_context(|| {
                    format!(
                        "Could not read default architecture file: {}",
                        default_path.display()
                    )
                })?;
                return Self::parse_json(&content);
            } else {
                anyhow::bail!(
                    "Architecture '{}' not found and no default architecture available. File: {}",
                    architecture_name,
                    architecture_path.display()
                );
            }
        }

        let content = fs::read_to_string(&architecture_path)
            .await
            .with_context(|| {
                format!(
                    "Could not read architecture file: {}",
                    architecture_path.display()
                )
            })?;

        Self::parse_json(&content)
    }

    /// List all available architectures in a directory
    #[allow(dead_code)]
    pub fn list_in_directory(architectures_dir: &Path) -> Result<Vec<String>> {
        let mut architectures = Vec::new();

        if !architectures_dir.exists() {
            return Ok(architectures);
        }

        for entry in std::fs::read_dir(architectures_dir)
            .with_context(|| format!("Failed to read directory: {}", architectures_dir.display()))?
        {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".json") && !name.starts_with('.') {
                        let arch_name = name.strip_suffix(".json").unwrap_or(name);
                        if arch_name != "default" {
                            // Skip default.json in listing
                            architectures.push(arch_name.to_string());
                        }
                    }
                }
            }
        }

        architectures.sort();
        Ok(architectures)
    }
}
