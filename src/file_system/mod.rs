//! # File System Module
//!
//! This module centralizes all file system operations for the CLI Frontend Generator.
//! It provides a clean abstraction over file I/O operations with proper error handling
//! and async support.
//!
//! ## Key Features
//!
//! - **Template Discovery**: Find and list available templates
//! - **File Operations**: Read templates and write generated files
//! - **Directory Management**: Create output directories
//! - **Error Handling**: Comprehensive error reporting for file operations
//!
//! ## Example Usage
//!
//! ```rust
//! use crate::file_system::FileSystem;
//!
//! let fs = FileSystem::new();
//! let content = fs.read_template(&template_path).await?;
//! fs.write_output(&output_path, &processed_content).await?;
//! ```

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::fs;
use walkdir::WalkDir;

use crate::types::{FileSystemError, FileSystemResult, TemplateContent};

/// File system operations handler
#[derive(Debug, Default)]
pub struct FileSystem;

impl FileSystem {
    /// Create a new FileSystem instance
    pub fn new() -> Self {
        Self
    }

    /// Read template file content
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the template file
    ///
    /// # Returns
    ///
    /// Template content wrapped in TemplateContent type
    ///
    /// # Errors
    ///
    /// Returns FileSystemError if the file cannot be read
    pub async fn read_template(&self, path: &Path) -> FileSystemResult<TemplateContent> {
        let content =
            fs::read_to_string(path).await.map_err(|e| FileSystemError::OperationFailed {
                operation: "read".to_string(),
                path: path.to_path_buf(),
                source: e,
            })?;

        Ok(TemplateContent::new(content))
    }

    /// Write generated content to output file
    ///
    /// # Arguments
    ///
    /// * `path` - Destination path for the output file
    /// * `content` - Content to write
    ///
    /// # Errors
    ///
    /// Returns FileSystemError if the file cannot be written
    pub async fn write_output(&self, path: &Path, content: &str) -> FileSystemResult<()> {
        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            self.create_directories(parent).await?;
        }

        fs::write(path, content).await.map_err(|e| FileSystemError::OperationFailed {
            operation: "write".to_string(),
            path: path.to_path_buf(),
            source: e,
        })?;

        Ok(())
    }

    /// Create directories recursively
    ///
    /// # Arguments
    ///
    /// * `path` - Directory path to create
    ///
    /// # Errors
    ///
    /// Returns FileSystemError if directories cannot be created
    pub async fn create_directories(&self, path: &Path) -> FileSystemResult<()> {
        fs::create_dir_all(path).await.map_err(|e| FileSystemError::DirectoryCreationFailed {
            path: path.to_path_buf(),
            source: e,
        })?;

        Ok(())
    }

    /// Discover available templates in the templates directory
    ///
    /// # Arguments
    ///
    /// * `templates_dir` - Path to the templates directory
    ///
    /// # Returns
    ///
    /// Vector of template names
    pub fn discover_templates(&self, templates_dir: &Path) -> Result<Vec<String>> {
        let mut templates = Vec::new();

        if !templates_dir.exists() {
            return Ok(templates);
        }

        for entry in std::fs::read_dir(templates_dir).with_context(|| {
            format!("Failed to read templates directory: {}", templates_dir.display())
        })? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    if !name.starts_with('.') {
                        templates.push(name.to_string());
                    }
                }
            }
        }

        templates.sort();
        Ok(templates)
    }

    /// Discover available architectures in the architectures directory
    ///
    /// # Arguments
    ///
    /// * `architectures_dir` - Path to the architectures directory
    ///
    /// # Returns
    ///
    /// Vector of architecture names (without .json extension)
    pub fn discover_architectures(&self, architectures_dir: &Path) -> Result<Vec<String>> {
        let mut architectures = Vec::new();

        if !architectures_dir.exists() {
            return Ok(architectures);
        }

        for entry in std::fs::read_dir(architectures_dir).with_context(|| {
            format!("Failed to read architectures directory: {}", architectures_dir.display())
        })? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".json") && !name.starts_with('.') {
                        let arch_name = name.strip_suffix(".json").unwrap_or(name);
                        if arch_name != "default" {
                            architectures.push(arch_name.to_string());
                        }
                    }
                }
            }
        }

        architectures.sort();
        Ok(architectures)
    }

    /// Walk through template directory and collect all files
    ///
    /// # Arguments
    ///
    /// * `template_dir` - Path to the template directory
    ///
    /// # Returns
    ///
    /// Vector of file paths relative to the template directory
    pub fn walk_template_directory(&self, template_dir: &Path) -> Result<Vec<(PathBuf, PathBuf)>> {
        let mut files = Vec::new();

        for entry in WalkDir::new(template_dir) {
            let entry = entry.with_context(|| "Error walking template directory")?;

            if entry.file_type().is_file() {
                // Skip .conf files
                if entry.file_name() == ".conf" {
                    continue;
                }

                let relative_path = entry
                    .path()
                    .strip_prefix(template_dir)
                    .with_context(|| "Could not get relative path")?;

                let template_file = entry.path().to_path_buf();
                files.push((template_file, relative_path.to_path_buf()));
            }
        }

        Ok(files)
    }

    /// Check if a template directory exists
    ///
    /// # Arguments
    ///
    /// * `templates_dir` - Base templates directory
    /// * `template_type` - Template type to check
    ///
    /// # Returns
    ///
    /// True if template directory exists
    pub fn template_exists(&self, templates_dir: &Path, template_type: &str) -> bool {
        templates_dir.join(template_type).exists()
    }

    /// List generated files in output directory
    ///
    /// # Arguments
    ///
    /// * `output_path` - Path to search for generated files
    ///
    /// # Returns
    ///
    /// Vector of generated file names
    pub fn list_generated_files(&self, output_path: &Path) -> Result<Vec<String>> {
        let mut files = Vec::new();

        for entry in WalkDir::new(output_path).max_depth(1) {
            let entry = entry.with_context(|| "Error reading output directory")?;

            if entry.file_type().is_file() {
                if let Some(filename) = entry.file_name().to_str() {
                    files.push(filename.to_string());
                }
            }
        }

        Ok(files)
    }

    /// Read configuration file content  
    ///
    /// # Arguments
    ///
    /// * `config_path` - Path to the configuration file
    ///
    /// # Returns
    ///
    /// Configuration file content as string
    pub async fn read_config_file(&self, config_path: &Path) -> FileSystemResult<String> {
        if !config_path.exists() {
            return Err(FileSystemError::FileNotFound { path: config_path.to_path_buf() });
        }

        let content = fs::read_to_string(config_path).await.map_err(|e| {
            FileSystemError::OperationFailed {
                operation: "read config".to_string(),
                path: config_path.to_path_buf(),
                source: e,
            }
        })?;

        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio::fs;

    #[tokio::test]
    async fn test_read_write_template() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let fs_handler = FileSystem::new();
        let test_content = "Hello, World!";

        // Write content
        fs_handler.write_output(&file_path, test_content).await.unwrap();

        // Read content back
        let content = fs_handler.read_template(&file_path).await.unwrap();
        assert_eq!(content.as_str(), test_content);
    }

    #[tokio::test]
    async fn test_create_directories() {
        let temp_dir = TempDir::new().unwrap();
        let nested_path = temp_dir.path().join("nested/deep/path");

        let fs_handler = FileSystem::new();
        fs_handler.create_directories(&nested_path).await.unwrap();

        assert!(nested_path.exists());
    }

    #[tokio::test]
    async fn test_discover_templates() {
        let temp_dir = TempDir::new().unwrap();
        let templates_dir = temp_dir.path();

        // Create some template directories
        fs::create_dir_all(templates_dir.join("component")).await.unwrap();
        fs::create_dir_all(templates_dir.join("hook")).await.unwrap();
        fs::create_dir_all(templates_dir.join(".hidden")).await.unwrap(); // Should be ignored

        let fs_handler = FileSystem::new();
        let templates = fs_handler.discover_templates(templates_dir).unwrap();

        assert_eq!(templates.len(), 2);
        assert!(templates.contains(&"component".to_string()));
        assert!(templates.contains(&"hook".to_string()));
        assert!(!templates.contains(&".hidden".to_string()));
    }

    #[test]
    fn test_template_exists() {
        let temp_dir = TempDir::new().unwrap();
        let templates_dir = temp_dir.path();

        // Create a template directory
        std::fs::create_dir_all(templates_dir.join("component")).unwrap();

        let fs_handler = FileSystem::new();
        assert!(fs_handler.template_exists(templates_dir, "component"));
        assert!(!fs_handler.template_exists(templates_dir, "nonexistent"));
    }
}
