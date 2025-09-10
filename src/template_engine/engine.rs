//! # Template Engine Core
//!
//! Core template processing logic with improved architecture and error handling.

use anyhow::{Context, Result};
use colored::*;
use handlebars::Handlebars;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::data_builder::TemplateDataBuilder;
use super::helpers::register_all_helpers;
use crate::config::{ArchitectureConfig, Config};
use crate::file_system::FileSystem;
use crate::naming::SmartNaming;
use crate::types::OutputPath;

/// Template configuration for individual templates
#[derive(Debug, Clone)]
pub struct TemplateConfig {
    pub variables: HashMap<String, String>,
    pub environment: String,
    pub enable_timestamps: bool,
    pub enable_uuid: bool,
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self {
            variables: HashMap::new(),
            environment: std::env::var("NODE_ENV").unwrap_or_else(|_| "development".to_string()),
            enable_timestamps: true,
            enable_uuid: true,
        }
    }
}

/// Main template engine responsible for code generation
pub struct TemplateEngine {
    templates_dir: PathBuf,
    output_dir: PathBuf,
    file_system: FileSystem,
    naming: SmartNaming,
}

impl TemplateEngine {
    /// Create a new TemplateEngine instance
    ///
    /// # Arguments
    ///
    /// * `templates_dir` - Directory containing template files
    /// * `output_dir` - Directory where generated files will be written
    pub fn new(templates_dir: PathBuf, output_dir: PathBuf) -> Result<Self> {
        Ok(Self {
            templates_dir,
            output_dir,
            file_system: FileSystem::new(),
            naming: SmartNaming::new(),
        })
    }

    /// Check if a template type exists
    pub fn template_exists(&self, template_type: &str) -> bool {
        self.file_system.template_exists(&self.templates_dir, template_type)
    }

    /// List available templates
    pub fn list_templates(&self) -> Result<Vec<String>> {
        self.file_system.discover_templates(&self.templates_dir)
    }

    /// Generate files using a template
    ///
    /// # Arguments
    ///
    /// * `name` - Name for the generated files/components
    /// * `template_type` - Type of template to use
    /// * `create_folder` - Whether to create a folder for the generated files
    pub async fn generate(
        &self,
        name: &str,
        template_type: &str,
        create_folder: bool,
    ) -> Result<()> {
        let template_dir = self.templates_dir.join(template_type);

        if !template_dir.exists() {
            return Err(anyhow::anyhow!(
                "Template directory not found: {}",
                template_dir.display()
            ));
        }

        // Load template configuration
        let template_config = self.load_template_config(template_type).await?;

        self.print_generation_info(template_type, &template_config);

        // Determine output path
        let output_path = self.determine_output_path(name, create_folder);

        // Create output directory
        self.file_system
            .create_directories(output_path.as_path())
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create output directory: {}", e))?;

        // Process all template files
        self.process_template_directory(&template_dir, &output_path, name, &template_config)
            .await?;

        // Show generated files
        self.show_generated_files(&output_path).await?;

        Ok(())
    }

    /// Generate a feature with specific architecture
    pub async fn generate_feature(
        &self,
        name: &str,
        architecture: Option<&str>,
        create_folder: bool,
        config: &Config,
    ) -> Result<()> {
        let architecture_name = architecture.unwrap_or(&config.default_architecture);

        // Load architecture configuration
        let arch_config = config
            .load_architecture(architecture_name)
            .await
            .with_context(|| format!("Failed to load architecture: {}", architecture_name))?;

        self.print_architecture_info(&arch_config);

        // Determine output path
        let output_path = self.determine_output_path(name, create_folder);

        // Create output directory
        self.file_system
            .create_directories(output_path.as_path())
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create output directory: {}", e))?;

        // Generate each structure defined in the architecture
        for structure in &arch_config.structure {
            self.generate_feature_structure(name, structure, output_path.as_path())
                .await
                .with_context(|| format!("Failed to generate structure: {}", structure.path))?;
        }

        // Show generated files
        self.show_generated_feature_files(&output_path, &arch_config).await?;

        Ok(())
    }

    /// Load template configuration from .conf file if exists
    async fn load_template_config(&self, template_type: &str) -> Result<TemplateConfig> {
        let config_path = self.templates_dir.join(template_type).join(".conf");

        if !config_path.exists() {
            return Ok(TemplateConfig::default());
        }

        let content = self
            .file_system
            .read_config_file(&config_path)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to read template config: {}", e))?;

        self.parse_template_config(&content)
    }

    /// Parse template configuration from INI-like format
    fn parse_template_config(&self, content: &str) -> Result<TemplateConfig> {
        let mut config = TemplateConfig::default();

        for line in content.lines() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            // Parse key=value pairs
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"').trim_matches('\'');

                match key {
                    "environment" => config.environment = value.to_string(),
                    "enable_timestamps" => config.enable_timestamps = value.parse().unwrap_or(true),
                    "enable_uuid" => config.enable_uuid = value.parse().unwrap_or(true),
                    _ => {
                        // Custom variables
                        if key.starts_with("var_") {
                            let var_name = key.strip_prefix("var_").unwrap_or(key);
                            config.variables.insert(var_name.to_string(), value.to_string());
                        }
                    },
                }
            }
        }

        Ok(config)
    }

    /// Determine the output path based on configuration
    fn determine_output_path(&self, name: &str, create_folder: bool) -> OutputPath {
        if create_folder {
            OutputPath::new(self.output_dir.join(name))
        } else {
            OutputPath::new(self.output_dir.clone())
        }
    }

    /// Process all files in a template directory
    async fn process_template_directory(
        &self,
        template_dir: &Path,
        output_path: &OutputPath,
        name: &str,
        template_config: &TemplateConfig,
    ) -> Result<()> {
        let files = self.file_system.walk_template_directory(template_dir)?;
        let mut tasks = Vec::new();

        for (template_file, relative_path) in files {
            let output_file = output_path.as_path().join(relative_path);
            let name_clone = name.to_string();
            let config_clone = template_config.clone();

            let file_system = FileSystem::new();
            let naming = SmartNaming::new();

            let task = tokio::spawn(async move {
                Self::process_single_template_file(
                    &file_system,
                    &naming,
                    &template_file,
                    &output_file,
                    &name_clone,
                    &config_clone,
                )
                .await
            });

            tasks.push(task);
        }

        // Wait for all files to be processed
        for task in tasks {
            task.await??;
        }

        Ok(())
    }

    /// Process a single template file
    async fn process_single_template_file(
        file_system: &FileSystem,
        naming: &SmartNaming,
        template_file: &Path,
        output_file: &Path,
        name: &str,
        template_config: &TemplateConfig,
    ) -> Result<()> {
        // Read template content
        let template_content = file_system
            .read_template(template_file)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to read template: {}", e))?;

        // Create handlebars instance with all helpers
        let mut handlebars = Handlebars::new();
        register_all_helpers(&mut handlebars);

        // Process smart names
        let smart_names = naming.process_smart_names(name);

        // Build template data
        let data = TemplateDataBuilder::new()
            .with_name(name)
            .with_environment(&template_config.environment)
            .with_timestamps(template_config.enable_timestamps)
            .with_uuid(template_config.enable_uuid)
            .with_variables(template_config.variables.clone())
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build template data: {}", e))?;

        // Apply smart replacements to content
        let processed_content =
            naming.apply_smart_replacements(template_content.as_str(), name, &smart_names);

        // Render with handlebars
        let rendered_content = handlebars
            .render_template(&processed_content, &data)
            .with_context(|| "Template rendering failed")?;

        // Process output filename with smart replacements
        let output_filename = output_file
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| naming.apply_smart_filename_replacements(n, name, &smart_names))
            .context("Invalid output filename")?;

        let final_output_path =
            output_file.parent().context("Invalid output path")?.join(output_filename);

        // Write processed content
        file_system
            .write_output(&final_output_path, &rendered_content)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to write output file: {}", e))?;

        Ok(())
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
            self.file_system
                .create_directories(&structure_path)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to create structure directory: {}", e))?;
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
        let processed_filename =
            self.naming.process_filename_pattern(&structure.filename_pattern, name);

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

    /// Process template directory for feature generation
    async fn process_feature_template_directory(
        &self,
        template_dir: &Path,
        output_path: &Path,
        name: &str,
        _filename_prefix: &str,
    ) -> Result<()> {
        let files = self.file_system.walk_template_directory(template_dir)?;
        let mut tasks = Vec::new();
        let smart_names = self.naming.process_smart_names(name);

        for (template_file, relative_path) in files {
            // Process output filename using the actual filename pattern
            let output_filename = if let Some(original_name) = relative_path.file_name() {
                let original_str = original_name.to_str().unwrap_or("");
                self.naming.apply_smart_filename_replacements(original_str, name, &smart_names)
            } else {
                format!("{}.ts", name)
            };

            let output_file = output_path.join(output_filename);
            let name_clone = name.to_string();

            let file_system = FileSystem::new();
            let naming = SmartNaming::new();

            let task = tokio::spawn(async move {
                Self::process_template_file_simple(
                    &file_system,
                    &naming,
                    &template_file,
                    &output_file,
                    &name_clone,
                )
                .await
            });

            tasks.push(task);
        }

        // Wait for all files to be processed
        for task in tasks {
            task.await??;
        }

        Ok(())
    }

    /// Simple template file processing for backward compatibility
    async fn process_template_file_simple(
        file_system: &FileSystem,
        naming: &SmartNaming,
        template_file: &Path,
        output_file: &Path,
        name: &str,
    ) -> Result<()> {
        let default_config = TemplateConfig::default();
        Self::process_single_template_file(
            file_system,
            naming,
            template_file,
            output_file,
            name,
            &default_config,
        )
        .await
    }

    /// Print generation information
    fn print_generation_info(&self, _template_type: &str, template_config: &TemplateConfig) {
        println!(
            "{} Using template config: environment={}",
            "⚙️".bold(),
            template_config.environment.blue()
        );
    }

    /// Print architecture information
    fn print_architecture_info(&self, arch_config: &ArchitectureConfig) {
        println!("{} Using {} architecture", "📐".bold(), arch_config.name.bold());
    }

    /// Show generated files for regular templates
    async fn show_generated_files(&self, output_path: &OutputPath) -> Result<()> {
        let files = self.file_system.list_generated_files(output_path.as_path())?;

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
        output_path: &OutputPath,
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
                output_path.as_path().to_path_buf()
            } else {
                output_path.as_path().join(&structure.path)
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

    /// Check if a template type is a feature template
    #[allow(dead_code)]
    pub fn is_feature_template(&self, template_type: &str) -> bool {
        template_type == "feature"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio::fs;

    #[tokio::test]
    async fn test_template_engine_creation() {
        let temp_dir = TempDir::new().unwrap();
        let templates_dir = temp_dir.path().join("templates");
        let output_dir = temp_dir.path().join("output");

        let engine = TemplateEngine::new(templates_dir, output_dir);
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn test_template_exists() {
        let temp_dir = TempDir::new().unwrap();
        let templates_dir = temp_dir.path().join("templates");
        let output_dir = temp_dir.path().join("output");

        // Create a template directory
        fs::create_dir_all(templates_dir.join("component")).await.unwrap();

        let engine = TemplateEngine::new(templates_dir, output_dir).unwrap();
        assert!(engine.template_exists("component"));
        assert!(!engine.template_exists("nonexistent"));
    }

    #[tokio::test]
    async fn test_list_templates() {
        let temp_dir = TempDir::new().unwrap();
        let templates_dir = temp_dir.path().join("templates");
        let output_dir = temp_dir.path().join("output");

        // Create template directories
        fs::create_dir_all(templates_dir.join("component")).await.unwrap();
        fs::create_dir_all(templates_dir.join("hook")).await.unwrap();

        let engine = TemplateEngine::new(templates_dir, output_dir).unwrap();
        let templates = engine.list_templates().unwrap();

        assert_eq!(templates.len(), 2);
        assert!(templates.contains(&"component".to_string()));
        assert!(templates.contains(&"hook".to_string()));
    }

    #[test]
    fn test_template_config_parsing() {
        let engine = TemplateEngine::new(PathBuf::new(), PathBuf::new()).unwrap();

        let config_content = r#"
            # Test configuration
            environment=production
            enable_timestamps=false
            enable_uuid=true
            var_author=John Doe
            var_license=MIT
        "#;

        let config = engine.parse_template_config(config_content).unwrap();

        assert_eq!(config.environment, "production");
        assert_eq!(config.enable_timestamps, false);
        assert_eq!(config.enable_uuid, true);
        assert_eq!(config.variables.get("author"), Some(&"John Doe".to_string()));
        assert_eq!(config.variables.get("license"), Some(&"MIT".to_string()));
    }
}
