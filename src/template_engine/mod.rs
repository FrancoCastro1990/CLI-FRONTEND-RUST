//! Template engine for generating frontend code from templates.
//!
//! This module provides the main `TemplateEngine` struct and its associated
//! functionality for processing templates, generating code, and managing
//! template configurations.
//!
//! # Overview
//!
//! The template engine uses Handlebars for template rendering and provides
//! smart name transformations, variable substitution, and conditional file
//! generation.
//!
//! # Example
//!
//! ```no_run
//! use cli_frontend::template_engine::TemplateEngine;
//! use std::path::PathBuf;
//! use std::collections::HashMap;
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! let engine = TemplateEngine::new(
//!     PathBuf::from("./templates"),
//!     PathBuf::from("./output")
//! )?;
//!
//! // Generate a component
//! engine.generate(
//!     "Button",
//!     "component",
//!     true,
//!     HashMap::new()
//! ).await?;
//! # Ok(())
//! # }
//! ```

pub mod config;
mod generator;
mod handlebars_renderer;
pub mod helpers;
mod inspector;
pub mod naming;
pub mod renderer;
mod renderer_trait;

// Re-export public types
pub use config::TemplateConfig;
#[allow(unused_imports)] // Used in doctests
pub use config::{TemplateMetadata, VariableOption};
#[allow(unused_imports)] // Public API for future use
pub use handlebars_renderer::HandlebarsRenderer;
#[allow(unused_imports)] // Public API for future use
pub use renderer_trait::TemplateRenderer;

use anyhow::{Context, Result};
use colored::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use walkdir::WalkDir;

use crate::config::{ArchitectureConfig, Config};
use generator::{
    evaluate_file_condition, merge_variables, prepare_output_directory, validate_template_exists,
};
use inspector::{
    print_file_filters, print_optional_variables, print_required_variables, print_template_header,
    print_usage_examples,
};
use naming::{apply_smart_filename_replacements, apply_smart_replacements, process_smart_names};
use renderer::{
    create_handlebars, create_template_data, determine_output_path, read_template, render_template,
    write_output,
};

/// Engine for processing and generating templates.
///
/// The `TemplateEngine` is the main entry point for template generation.
/// It manages template loading, variable substitution, Handlebars rendering,
/// and file generation with support for conditional file creation.
///
/// # Example
///
/// ```no_run
/// # use cli_frontend::template_engine::TemplateEngine;
/// # use std::path::PathBuf;
/// # use std::collections::HashMap;
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// let engine = TemplateEngine::new(
///     PathBuf::from("./templates"),
///     PathBuf::from("./src/components")
/// )?;
///
/// let mut vars = HashMap::new();
/// vars.insert("style".to_string(), "scss".to_string());
/// vars.insert("with_tests".to_string(), "true".to_string());
///
/// engine.generate("Button", "component", true, vars).await?;
/// # Ok(())
/// # }
/// ```
pub struct TemplateEngine {
    templates_dir: PathBuf,
    output_dir: PathBuf,
}

impl TemplateEngine {
    /// Creates a new TemplateEngine instance.
    ///
    /// # Arguments
    ///
    /// * `templates_dir` - Path to the directory containing template folders
    /// * `output_dir` - Base directory where generated files will be written
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the new `TemplateEngine` instance.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use cli_frontend::template_engine::TemplateEngine;
    /// # use std::path::PathBuf;
    /// let engine = TemplateEngine::new(
    ///     PathBuf::from("./templates"),
    ///     PathBuf::from("./output")
    /// )?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn new(templates_dir: PathBuf, output_dir: PathBuf) -> Result<Self> {
        Ok(Self {
            templates_dir,
            output_dir,
        })
    }

    /// Checks if a template type exists in the templates directory.
    ///
    /// # Arguments
    ///
    /// * `template_type` - Name of the template to check (e.g., "component", "hook")
    ///
    /// # Returns
    ///
    /// Returns `true` if the template directory exists, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use cli_frontend::template_engine::TemplateEngine;
    /// # use std::path::PathBuf;
    /// # let engine = TemplateEngine::new(PathBuf::from("./templates"), PathBuf::from("./output"))?;
    /// if engine.template_exists("component") {
    ///     println!("Component template found!");
    /// }
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn template_exists(&self, template_type: &str) -> bool {
        self.templates_dir.join(template_type).exists()
    }

    /// Lists all available template types.
    ///
    /// Scans the templates directory and returns a sorted vector of template names.
    /// Hidden directories (starting with '.') are excluded.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of template names.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use cli_frontend::template_engine::TemplateEngine;
    /// # use std::path::PathBuf;
    /// # let engine = TemplateEngine::new(PathBuf::from("./templates"), PathBuf::from("./output"))?;
    /// let templates = engine.list_templates()?;
    /// for template in templates {
    ///     println!("Available: {}", template);
    /// }
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn list_templates(&self) -> Result<Vec<String>> {
        let mut templates = Vec::new();

        if !self.templates_dir.exists() {
            return Ok(templates);
        }

        for entry in std::fs::read_dir(&self.templates_dir)? {
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

    /// Generates code from a template with the given name and type.
    ///
    /// This method processes a template directory, applies variable substitutions,
    /// renders files with Handlebars, and writes the output to the specified directory.
    ///
    /// # Arguments
    ///
    /// * `name` - The name for the generated code (e.g., "Button", "useAuth")
    /// * `template_type` - The type of template to use (e.g., "component", "hook")
    /// * `create_folder` - Whether to create a subfolder with the component name
    /// * `cli_vars` - Additional variables to pass to the template
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The template directory doesn't exist
    /// - Template configuration is invalid
    /// - File I/O operations fail
    /// - Template rendering fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use cli_frontend::template_engine::TemplateEngine;
    /// # use std::path::PathBuf;
    /// # use std::collections::HashMap;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let engine = TemplateEngine::new(
    ///     PathBuf::from("./templates"),
    ///     PathBuf::from("./output")
    /// )?;
    ///
    /// let mut vars = HashMap::new();
    /// vars.insert("style".to_string(), "scss".to_string());
    ///
    /// engine.generate("Button", "component", true, vars).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generate(
        &self,
        name: &str,
        template_type: &str,
        create_folder: bool,
        cli_vars: std::collections::HashMap<String, String>,
    ) -> Result<()> {
        let template_dir = validate_template_exists(&self.templates_dir, template_type)?;
        let mut template_config = self.load_template_config(template_type).await?;
        merge_variables(cli_vars, &mut template_config);

        let output_path = prepare_output_directory(&self.output_dir, name, create_folder).await?;

        self.process_template_directory(&template_dir, &output_path, name, &template_config)
            .await?;
        self.show_generated_files(&output_path).await?;

        Ok(())
    }

    /// Generates a complete feature with a specific architecture pattern.
    ///
    /// Creates a full feature structure following an architectural pattern
    /// (e.g., Clean Architecture, Redux, MVC). Each architecture defines
    /// a directory structure with specific templates for each layer.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the feature (e.g., "Authentication", "PaymentSystem")
    /// * `architecture` - Optional architecture name. If None, uses default from config
    /// * `create_folder` - Whether to create a subfolder with the feature name
    /// * `config` - Application configuration containing architecture definitions
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The architecture configuration doesn't exist
    /// - Required templates are missing
    /// - Directory creation fails
    /// - Template processing fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use cli_frontend::template_engine::TemplateEngine;
    /// # use cli_frontend::config::Config;
    /// # use std::path::PathBuf;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let engine = TemplateEngine::new(
    ///     PathBuf::from("./templates"),
    ///     PathBuf::from("./src/features")
    /// )?;
    ///
    /// let config = Config::load(&None).await?;
    ///
    /// // Generate with Clean Architecture
    /// engine.generate_feature(
    ///     "PaymentSystem",
    ///     Some("clean-architecture"),
    ///     true,
    ///     &config
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generate_feature(
        &self,
        name: &str,
        architecture: Option<&str>,
        create_folder: bool,
        config: &Config,
    ) -> Result<()> {
        let architecture_name = architecture.unwrap_or(config.default_architecture());

        // Load architecture configuration
        let arch_config = config
            .load_architecture(architecture_name)
            .await
            .with_context(|| format!("Failed to load architecture: {}", architecture_name))?;

        println!(
            "{} Using {} architecture",
            "📐".bold(),
            arch_config.name.bold()
        );

        // Determine output path
        let output_path = if create_folder {
            self.output_dir.join(name)
        } else {
            self.output_dir.clone()
        };

        // Create output directory
        fs::create_dir_all(&output_path).await.with_context(|| {
            format!(
                "Could not create output directory: {}",
                output_path.display()
            )
        })?;

        // Generate each structure defined in the architecture
        for structure in &arch_config.structure {
            self.generate_feature_structure(name, structure, &output_path)
                .await
                .with_context(|| format!("Failed to generate structure: {}", structure.path))?;
        }

        // Show generated files
        self.show_generated_feature_files(&output_path, &arch_config)
            .await?;

        Ok(())
    }

    /// Displays detailed information about a template.
    ///
    /// Shows template metadata, available variables with types and defaults,
    /// file generation rules, and usage examples. This is useful for exploring
    /// templates before using them.
    ///
    /// # Arguments
    ///
    /// * `template_type` - Name of the template to describe
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success.
    ///
    /// # Errors
    ///
    /// Returns an error if the template doesn't exist.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use cli_frontend::template_engine::TemplateEngine;
    /// # use std::path::PathBuf;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let engine = TemplateEngine::new(
    ///     PathBuf::from("./templates"),
    ///     PathBuf::from("./output")
    /// )?;
    ///
    /// // Show detailed information about the component template
    /// engine.describe_template("component").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn describe_template(&self, template_type: &str) -> Result<()> {
        let config = self
            .load_template_config_for_describe(template_type)
            .await?;

        print_template_header(template_type, &config.metadata);

        if !config.options_metadata.is_empty() || !config.variables.is_empty() {
            println!("{}", "Template Variables (use --var):".bold().green());
            println!();
            print_required_variables(&config.options_metadata, &config.variables);
            print_optional_variables(&config.variables, &config.options_metadata);
        }

        if !config.file_filters.is_empty() {
            print_file_filters(&config.file_filters);
        }

        print_usage_examples(template_type, &config);

        Ok(())
    }

    // ============ Private Methods ============

    /// Load template configuration from .conf file if exists
    async fn load_template_config(&self, template_type: &str) -> Result<TemplateConfig> {
        let config_path = self.templates_dir.join(template_type).join(".conf");

        if !config_path.exists() {
            return Ok(TemplateConfig::default());
        }

        let content = fs::read_to_string(&config_path).await.with_context(|| {
            format!("Could not read template config: {}", config_path.display())
        })?;

        let config = self.parse_template_config(&content)?;

        Ok(config)
    }

    /// Parse template configuration from INI-like format with sections
    fn parse_template_config(&self, content: &str) -> Result<TemplateConfig> {
        let mut config = TemplateConfig::default();
        let mut current_section = String::new();

        for line in content.lines() {
            let line = line.trim();

            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len() - 1].to_string();
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.split('#').next().unwrap_or(value);
                let value = value.trim().trim_matches('"').trim_matches('\'');

                match current_section.as_str() {
                    "metadata" => Self::parse_metadata_section(&mut config, key, value),
                    "options" => Self::parse_options_section(&mut config, key, value),
                    "files" => {
                        config
                            .file_filters
                            .insert(key.to_string(), value.to_string());
                    }
                    _ => Self::parse_root_config(&mut config, key, value),
                }
            }
        }

        Ok(config)
    }

    /// Parse options section of template config
    fn parse_options_section(config: &mut TemplateConfig, key: &str, value: &str) {
        if let Some(var_name) = key.strip_suffix("_options") {
            let possible_values: Vec<String> = value
                .split(',')
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty())
                .collect();

            config
                .options_metadata
                .entry(var_name.to_string())
                .or_default()
                .possible_values = possible_values;
        } else if let Some(var_name) = key.strip_suffix("_type") {
            config
                .options_metadata
                .entry(var_name.to_string())
                .or_default()
                .var_type = value.to_string();
        } else if let Some(var_name) = key.strip_suffix("_description") {
            config
                .options_metadata
                .entry(var_name.to_string())
                .or_default()
                .description = value.to_string();
        } else {
            config.variables.insert(key.to_string(), value.to_string());
        }
    }

    /// Parse metadata section of template config
    fn parse_metadata_section(config: &mut TemplateConfig, key: &str, value: &str) {
        match key {
            "name" => config.metadata.name = value.to_string(),
            "description" => config.metadata.description = value.to_string(),
            _ => {}
        }
    }

    /// Parse root-level config keys
    fn parse_root_config(config: &mut TemplateConfig, key: &str, value: &str) {
        match key {
            "environment" => config.environment = value.to_string(),
            "enable_timestamps" => config.enable_timestamps = value.parse().unwrap_or(true),
            "enable_uuid" => config.enable_uuid = value.parse().unwrap_or(true),
            _ => {
                if let Some(var_name) = key.strip_prefix("var_") {
                    config
                        .variables
                        .insert(var_name.to_string(), value.to_string());
                }
            }
        }
    }

    /// Load template configuration for describe command
    async fn load_template_config_for_describe(
        &self,
        template_type: &str,
    ) -> Result<TemplateConfig> {
        if !self.template_exists(template_type) {
            anyhow::bail!(
                "Template '{}' not found.\n\nRun {} to see available templates.",
                template_type.red(),
                "cli-frontend --list".cyan()
            );
        }
        self.load_template_config(template_type).await
    }

    /// Process template directory for standard generation
    async fn process_template_directory(
        &self,
        template_dir: &Path,
        output_path: &Path,
        name: &str,
        template_config: &TemplateConfig,
    ) -> Result<()> {
        let mut tasks = Vec::new();
        let config_arc = Arc::new(template_config.clone());

        // Walk through all files in template directory
        for entry in WalkDir::new(template_dir) {
            let entry = entry.context("Error walking template directory")?;

            if entry.file_type().is_file() {
                // Skip .conf files
                if entry.file_name() == ".conf" {
                    continue;
                }

                let relative_path = entry
                    .path()
                    .strip_prefix(template_dir)
                    .context("Could not get relative path")?;

                // Get the filename as a string for filter matching
                let filename = relative_path.to_str().unwrap_or("").replace('\\', "/"); // Normalize path separators

                // Check if this file should be generated based on filters
                let should_generate = if !config_arc.file_filters.is_empty() {
                    // If file_filters exist, check if there's a condition for this file
                    if let Some(condition) = config_arc.file_filters.get(&filename) {
                        evaluate_file_condition(condition, &config_arc.variables)
                    } else {
                        // No explicit filter for this file, default to true
                        true
                    }
                } else {
                    // No file_filters defined, generate all files
                    true
                };

                if !should_generate {
                    continue;
                }

                let template_file = entry.path().to_path_buf();
                let output_file = output_path.join(relative_path);

                // Process file asynchronously - use Arc::clone for cheap reference counting
                let name_clone = name.to_string();
                let config_ref = Arc::clone(&config_arc);
                let task = tokio::spawn(async move {
                    Self::process_template_file_with_config(
                        &template_file,
                        &output_file,
                        &name_clone,
                        &config_ref,
                    )
                    .await
                });

                tasks.push(task);
            }
        }

        // Wait for all files to be processed
        for task in tasks {
            task.await??;
        }

        Ok(())
    }

    /// Process a single template file with configuration
    async fn process_template_file_with_config(
        template_file: &Path,
        output_file: &Path,
        name: &str,
        template_config: &TemplateConfig,
    ) -> Result<()> {
        let template_content = read_template(template_file).await?;
        let handlebars = create_handlebars();
        let data = create_template_data(name, template_config);

        let processed_names = process_smart_names(name);
        let processed_content = apply_smart_replacements(&template_content, name, &processed_names);

        let rendered_content = render_template(&handlebars, &processed_content, &data)?;
        let final_output_path = determine_output_path(output_file, name, &processed_names)?;

        write_output(&final_output_path, &rendered_content).await
    }

    /// Generate a single structure part of a feature
    async fn generate_feature_structure(
        &self,
        name: &str,
        structure: &crate::config::ArchitectureStructure,
        base_output_path: &Path,
    ) -> Result<()> {
        // Create the specific path for this structure
        let structure_path = if structure.path.is_empty() {
            base_output_path.to_path_buf()
        } else {
            base_output_path.join(&structure.path)
        };

        // Create directory if needed
        if !structure.path.is_empty() {
            fs::create_dir_all(&structure_path).await.with_context(|| {
                format!(
                    "Could not create structure directory: {}",
                    structure_path.display()
                )
            })?;
        }

        // Get template directory
        let template_dir = self.templates_dir.join(&structure.template);

        if !template_dir.exists() {
            return Err(anyhow::anyhow!(
                "Template '{}' not found for structure '{}'. Expected at: {}",
                structure.template,
                structure.path,
                template_dir.display()
            ));
        }

        // Process filename pattern
        let processed_filename = self.process_filename_pattern(&structure.filename_pattern, name);

        // Process all template files
        self.process_feature_template_directory(
            &template_dir,
            &structure_path,
            name,
            &processed_filename,
        )
        .await?;

        Ok(())
    }

    /// Process filename pattern with smart replacements
    fn process_filename_pattern(&self, pattern: &str, name: &str) -> String {
        let smart_names = process_smart_names(name);

        let mut result = pattern.to_string();

        // Replace specific patterns
        result = result.replace("use{name}", &smart_names.hook_name);
        result = result.replace("{name}Context", &smart_names.context_name);
        result = result.replace("{name}Provider", &smart_names.provider_name);
        result = result.replace("{name}Page", &smart_names.page_name);

        // Replace remaining {name}
        result = result.replace("{name}", name);

        result
    }

    /// Process template directory for feature generation
    async fn process_feature_template_directory(
        &self,
        template_dir: &Path,
        output_path: &Path,
        name: &str,
        filename_prefix: &str,
    ) -> Result<()> {
        let mut tasks = Vec::new();
        let smart_names = process_smart_names(name);

        // Walk through all files in template directory
        for entry in WalkDir::new(template_dir) {
            let entry = entry.context("Error walking template directory")?;

            if entry.file_type().is_file() {
                let relative_path = entry
                    .path()
                    .strip_prefix(template_dir)
                    .context("Could not get relative path")?;

                let template_file = entry.path().to_path_buf();

                // Process output filename - use the pattern from the original template name
                let output_filename = if let Some(original_name) = relative_path.file_name() {
                    let original_str = original_name.to_str().unwrap_or("");

                    // Apply smart filename replacements using the actual filename pattern
                    apply_smart_filename_replacements(original_str, name, &smart_names)
                } else {
                    format!("{}.ts", filename_prefix)
                };

                let output_file = output_path.join(output_filename);

                // Process file asynchronously
                let name_clone = name.to_string();
                let task = tokio::spawn(async move {
                    Self::process_template_file(&template_file, &output_file, &name_clone).await
                });

                tasks.push(task);
            }
        }

        // Wait for all files to be processed
        for task in tasks {
            task.await??;
        }

        Ok(())
    }

    /// Process template file (backward compatibility)
    async fn process_template_file(
        template_file: &Path,
        output_file: &Path,
        name: &str,
    ) -> Result<()> {
        // Use default config for backward compatibility
        let default_config = TemplateConfig::default();
        Self::process_template_file_with_config(template_file, output_file, name, &default_config)
            .await
    }

    /// Show generated files for standard generation
    async fn show_generated_files(&self, output_path: &Path) -> Result<()> {
        let mut files = Vec::new();

        for entry in WalkDir::new(output_path).max_depth(1) {
            let entry = entry.context("Error reading output directory")?;

            if entry.file_type().is_file() {
                if let Some(filename) = entry.file_name().to_str() {
                    files.push(filename.to_string());
                }
            }
        }

        if !files.is_empty() {
            println!("{}", "Files created:".bold());
            for file in files {
                println!("  - {}", file.green());
            }
        }

        Ok(())
    }

    /// Show generated feature files with architecture info
    async fn show_generated_feature_files(
        &self,
        output_path: &Path,
        arch_config: &ArchitectureConfig,
    ) -> Result<()> {
        println!("{}", "📁 Feature structure created:".bold());
        println!("  Architecture: {}", arch_config.name.green());
        println!("  Description: {}", arch_config.description);
        println!();

        // Show structure
        for structure in &arch_config.structure {
            println!("  📂 {} - {}", structure.path.blue(), structure.description);

            // List files in this structure
            let structure_path = if structure.path.is_empty() {
                output_path.to_path_buf()
            } else {
                output_path.join(&structure.path)
            };

            if structure_path.exists() {
                if let Ok(entries) = std::fs::read_dir(&structure_path) {
                    for entry in entries.flatten() {
                        if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                            if let Some(filename) = entry.file_name().to_str() {
                                println!("     📄 {}", filename.green());
                            }
                        }
                    }
                }
            }
        }

        println!();
        println!("{}", "Benefits:".bold());
        for benefit in &arch_config.benefits {
            println!("  ✅ {}", benefit);
        }

        if !arch_config.limitations.is_empty() {
            println!();
            println!("{}", "Considerations:".bold());
            for limitation in &arch_config.limitations {
                println!("  ⚠️  {}", limitation);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_config_default() {
        let config = TemplateConfig::default();
        assert!(config.enable_timestamps);
        assert!(config.enable_uuid);
        assert!(config.variables.is_empty());
        assert!(config.file_filters.is_empty());
    }
}
