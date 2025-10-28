use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use colored::*;
use handlebars::{Handlebars, Helper, HelperResult, Output, RenderContext};
use serde_json::json;
use std::path::{Path, PathBuf};
use tokio::fs;
use uuid::Uuid;
use walkdir::WalkDir;

use crate::config::{ArchitectureConfig, Config};

#[derive(Debug)]
struct SmartNames {
    hook_name: String,
    context_name: String,
    provider_name: String,
    page_name: String,
}

#[derive(Debug, Clone)]
pub struct TemplateConfig {
    pub variables: std::collections::HashMap<String, String>,
    pub environment: String,
    pub enable_timestamps: bool,
    pub enable_uuid: bool,
    /// Maps filename pattern to condition (e.g., "$FILE_NAME.spec.tsx" -> "var_with_tests")
    pub file_filters: std::collections::HashMap<String, String>,
    /// Template metadata
    pub metadata: TemplateMetadata,
    /// Metadata about each variable option (for dynamic boolean helper generation)
    pub options_metadata: std::collections::HashMap<String, VariableOption>,
}

#[derive(Debug, Clone)]
pub struct TemplateMetadata {
    pub name: String,
    pub description: String,
}

/// Metadata about a variable option from the .conf file
#[derive(Debug, Clone)]
pub struct VariableOption {
    /// Type of variable: "boolean", "string", "enum", etc.
    pub var_type: String,
    /// Possible values (from {var}_options in .conf)
    pub possible_values: Vec<String>,
    /// Description of the variable
    pub description: String,
}

impl Default for TemplateMetadata {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
        }
    }
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self {
            variables: std::collections::HashMap::new(),
            environment: std::env::var("NODE_ENV").unwrap_or_else(|_| "development".to_string()),
            enable_timestamps: true,
            enable_uuid: true,
            file_filters: std::collections::HashMap::new(),
            metadata: TemplateMetadata::default(),
            options_metadata: std::collections::HashMap::new(),
        }
    }
}

pub struct TemplateEngine {
    templates_dir: PathBuf,
    output_dir: PathBuf,
    // Removed handlebars field since we create it per-file for better flexibility
}

impl TemplateEngine {
    pub fn new(templates_dir: PathBuf, output_dir: PathBuf) -> Result<Self> {
        Ok(Self {
            templates_dir,
            output_dir,
        })
    }

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

            // Skip comments and empty lines
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            // Check for section headers [section_name]
            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len() - 1].to_string();
                continue;
            }

            // Parse key=value pairs
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                // Remove inline comments (everything after # not inside quotes)
                let value = if let Some(comment_pos) = value.find('#') {
                    &value[..comment_pos]
                } else {
                    value
                };
                let value = value.trim().trim_matches('"').trim_matches('\'');

                match current_section.as_str() {
                    "metadata" => {
                        // Parse metadata section
                        match key {
                            "name" => config.metadata.name = value.to_string(),
                            "description" => config.metadata.description = value.to_string(),
                            _ => {}
                        }
                    }
                    "options" => {
                        // Parse options section - these become default variable values
                        // AND capture metadata for dynamic boolean helper generation

                        if key.ends_with("_options") {
                            // Extract variable name: "style_options" -> "style"
                            let var_name = key.strip_suffix("_options").unwrap_or(key);

                            // Parse comma-separated list of possible values
                            let possible_values: Vec<String> = value
                                .split(',')
                                .map(|v| v.trim().to_string())
                                .filter(|v| !v.is_empty())
                                .collect();

                            // Get or create option metadata entry
                            let option = config.options_metadata
                                .entry(var_name.to_string())
                                .or_insert_with(|| VariableOption {
                                    var_type: String::new(),
                                    possible_values: Vec::new(),
                                    description: String::new(),
                                });

                            option.possible_values = possible_values;

                        } else if key.ends_with("_type") {
                            // Extract variable name: "with_tests_type" -> "with_tests"
                            let var_name = key.strip_suffix("_type").unwrap_or(key);

                            // Get or create option metadata entry
                            let option = config.options_metadata
                                .entry(var_name.to_string())
                                .or_insert_with(|| VariableOption {
                                    var_type: String::new(),
                                    possible_values: Vec::new(),
                                    description: String::new(),
                                });

                            option.var_type = value.to_string();

                        } else if key.ends_with("_description") {
                            // Extract variable name: "style_description" -> "style"
                            let var_name = key.strip_suffix("_description").unwrap_or(key);

                            // Get or create option metadata entry
                            let option = config.options_metadata
                                .entry(var_name.to_string())
                                .or_insert_with(|| VariableOption {
                                    var_type: String::new(),
                                    possible_values: Vec::new(),
                                    description: String::new(),
                                });

                            option.description = value.to_string();

                        } else {
                            // This is the actual variable default value
                            config.variables.insert(key.to_string(), value.to_string());
                        }
                    }
                    "files" => {
                        // Parse files section - file pattern -> condition mapping
                        config
                            .file_filters
                            .insert(key.to_string(), value.to_string());
                    }
                    _ => {
                        // Legacy format or root-level config
                        match key {
                            "environment" => config.environment = value.to_string(),
                            "enable_timestamps" => {
                                config.enable_timestamps = value.parse().unwrap_or(true)
                            }
                            "enable_uuid" => config.enable_uuid = value.parse().unwrap_or(true),
                            _ => {
                                // Custom variables (legacy var_ prefix)
                                if key.starts_with("var_") {
                                    let var_name = key.strip_prefix("var_").unwrap_or(key);
                                    config
                                        .variables
                                        .insert(var_name.to_string(), value.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(config)
    }

    /// Evaluate file condition to determine if a file should be generated
    ///
    /// Supported conditions:
    /// - "always" or "default" → always generate
    /// - "var_X" → generate if variable X is truthy (true, yes, 1)
    /// - "var_X_value" → generate if variable X equals "value"
    ///
    /// Examples:
    /// - "var_with_tests" → generate if with_tests=true
    /// - "var_style_scss" → generate if style=scss
    fn evaluate_file_condition(
        condition: &str,
        variables: &std::collections::HashMap<String, String>,
    ) -> bool {
        match condition.trim() {
            "always" | "default" => true,
            cond if cond.starts_with("var_") => {
                // Extract variable name and optional value check
                let var_part = cond.strip_prefix("var_").unwrap_or(cond);

                // First, check if this variable name exists as-is (for boolean checks)
                // e.g., "var_with_tests" → check variables["with_tests"]
                if variables.contains_key(var_part) {
                    // Boolean check: variable exists, check if it's truthy
                    variables.get(var_part).map_or(false, |v| {
                        matches!(v.to_lowercase().as_str(), "true" | "yes" | "1")
                    })
                } else if let Some(underscore_pos) = var_part.find('_') {
                    // Value comparison: var_style_scss → check if style == "scss"
                    // Handle hyphenated values: var_style_styled_components → check if style == "styled-components"
                    let var_name = &var_part[..underscore_pos];
                    let expected_value_raw = &var_part[underscore_pos + 1..];
                    // Convert underscores back to hyphens for value matching
                    let expected_value = expected_value_raw.replace('_', "-");

                    variables.get(var_name).map_or(false, |v| v == &expected_value || v == expected_value_raw)
                } else {
                    // Variable doesn't exist and no underscore found
                    false
                }
            }
            _ => {
                // Unknown condition, default to false for safety
                eprintln!(
                    "Warning: Unknown file condition '{}', skipping file",
                    condition
                );
                false
            }
        }
    }

    /// Generate boolean helper variables dynamically based on options metadata
    ///
    /// For each variable with `_options` in .conf, creates `{var}_is_{value}` boolean helpers.
    /// For each variable with `type=boolean` in .conf, creates `{var}_bool` boolean helper.
    ///
    /// Example:
    /// ```
    /// style=scss with style_options=scss,styled-components,css,none
    /// → generates: style_is_scss=true, style_is_styled_components=false, etc.
    ///
    /// with_tests=true with with_tests_type=boolean
    /// → generates: with_tests_bool=true
    /// ```
    fn generate_boolean_helpers(
        variables: &std::collections::HashMap<String, String>,
        options_metadata: &std::collections::HashMap<String, VariableOption>,
        data_map: &mut serde_json::Map<String, serde_json::Value>,
    ) {
        for (var_name, option_meta) in options_metadata {
            // Case 1: Variable with enumerated values (e.g., style_options=scss,css,none)
            if !option_meta.possible_values.is_empty() {
                if let Some(current_value) = variables.get(var_name) {
                    for possible_value in &option_meta.possible_values {
                        // Generate helper name: style_is_scss, style_is_styled_components, etc.
                        // Replace hyphens with underscores in the value part for valid variable names
                        let sanitized_value = possible_value.replace('-', "_");
                        let helper_name = format!("{}_is_{}", var_name, sanitized_value);
                        let is_match = current_value == possible_value;

                        data_map.insert(helper_name, serde_json::Value::Bool(is_match));
                    }
                }
            }

            // Case 2: Boolean variable (e.g., with_tests_type=boolean)
            if option_meta.var_type == "boolean" {
                if let Some(value) = variables.get(var_name) {
                    let is_true = matches!(
                        value.to_lowercase().as_str(),
                        "true" | "yes" | "1"
                    );
                    let helper_name = format!("{}_bool", var_name);

                    data_map.insert(helper_name, serde_json::Value::Bool(is_true));
                }
            }
        }
    }

    pub fn template_exists(&self, template_type: &str) -> bool {
        self.templates_dir.join(template_type).exists()
    }

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

    /// Check if a template type is a feature template
    #[allow(dead_code)]
    pub fn is_feature_template(&self, template_type: &str) -> bool {
        template_type == "feature"
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
        let smart_names = Self::process_smart_names(name);

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
        let smart_names = Self::process_smart_names(name);

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
                    Self::apply_smart_filename_replacements(original_str, name, &smart_names)
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

    pub async fn generate(
        &self,
        name: &str,
        template_type: &str,
        create_folder: bool,
        cli_vars: std::collections::HashMap<String, String>,
    ) -> Result<()> {
        let template_dir = self.templates_dir.join(template_type);

        if !template_dir.exists() {
            return Err(anyhow::anyhow!(
                "Template directory not found: {}",
                template_dir.display()
            ));
        }

        // Load template configuration
        let mut template_config = self.load_template_config(template_type).await?;

        // Merge CLI variables with template defaults (CLI variables override defaults)
        for (key, value) in cli_vars {
            template_config.variables.insert(key, value);
        }

        println!(
            "{} Using template config: environment={}",
            "⚙️".bold(),
            template_config.environment.blue()
        );

        // Show active variables if any were customized
        if !template_config.variables.is_empty() {
            println!("{} Active variables:", "🔧".bold());
            for (key, value) in &template_config.variables {
                println!("  {} = {}", key.cyan(), value.green());
            }
        }

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

        // Process all template files
        self.process_template_directory(&template_dir, &output_path, name, &template_config)
            .await?;

        // Show generated files
        self.show_generated_files(&output_path).await?;

        Ok(())
    }

    async fn process_template_directory(
        &self,
        template_dir: &Path,
        output_path: &Path,
        name: &str,
        template_config: &TemplateConfig,
    ) -> Result<()> {
        let mut tasks = Vec::new();

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
                let filename = relative_path
                    .to_str()
                    .unwrap_or("")
                    .replace('\\', "/"); // Normalize path separators

                // Check if this file should be generated based on filters
                let should_generate = if !template_config.file_filters.is_empty() {
                    // If file_filters exist, check if there's a condition for this file
                    if let Some(condition) = template_config.file_filters.get(&filename) {
                        Self::evaluate_file_condition(condition, &template_config.variables)
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

                // Process file asynchronously
                let name_clone = name.to_string();
                let config_clone = template_config.clone();
                let task = tokio::spawn(async move {
                    Self::process_template_file_with_config(
                        &template_file,
                        &output_file,
                        &name_clone,
                        &config_clone,
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

    async fn process_template_file_with_config(
        template_file: &Path,
        output_file: &Path,
        name: &str,
        template_config: &TemplateConfig,
    ) -> Result<()> {
        // Read template content
        let template_content = fs::read_to_string(template_file).await.with_context(|| {
            format!("Could not read template file: {}", template_file.display())
        })?;

        // Create handlebars instance for this file
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("pascal_case", Box::new(pascal_case_helper));
        handlebars.register_helper("snake_case", Box::new(snake_case_helper));
        handlebars.register_helper("kebab_case", Box::new(kebab_case_helper));
        handlebars.register_helper("camel_case", Box::new(camel_case_helper));
        handlebars.register_helper("upper_case", Box::new(upper_case_helper));
        handlebars.register_helper("timestamp", Box::new(timestamp_helper));
        handlebars.register_helper("uuid", Box::new(uuid_helper));
        handlebars.register_helper("env", Box::new(env_helper));
        handlebars.register_helper("eq", Box::new(eq_helper));
        handlebars.register_helper("ne", Box::new(ne_helper));

        // Smart name processing for different patterns
        let processed_names = Self::process_smart_names(name);

        // Get current timestamp and UUID
        let now: DateTime<Utc> = Utc::now();
        let current_uuid = Uuid::new_v4();

        // Prepare base template data
        let mut data = json!({
            "name": name,
            "pascal_name": to_pascal_case(name),
            "snake_name": to_snake_case(name),
            "kebab_name": to_kebab_case(name),
            "camel_name": to_camel_case(name),
            "upper_name": name.to_uppercase(),
            // Smart processed names
            "hook_name": processed_names.hook_name,
            "context_name": processed_names.context_name,
            "provider_name": processed_names.provider_name,
            "page_name": processed_names.page_name,
            // Environment-aware variables
            "environment": template_config.environment,
            "timestamp": if template_config.enable_timestamps { now.to_rfc3339() } else { "".to_string() },
            "timestamp_iso": if template_config.enable_timestamps { now.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string() } else { "".to_string() },
            "date": if template_config.enable_timestamps { now.format("%Y-%m-%d").to_string() } else { "".to_string() },
            "time": if template_config.enable_timestamps { now.format("%H:%M:%S").to_string() } else { "".to_string() },
            "year": if template_config.enable_timestamps { now.format("%Y").to_string() } else { "".to_string() },
            "uuid": if template_config.enable_uuid { current_uuid.to_string() } else { "".to_string() },
            "uuid_simple": if template_config.enable_uuid { current_uuid.simple().to_string() } else { "".to_string() },
            // Version info
            "version": env!("CARGO_PKG_VERSION"),
            "generator_name": "CLI Frontend Generator",
            "generated": true
        });

        // Add custom variables from template config
        if let Some(data_map) = data.as_object_mut() {
            for (key, value) in &template_config.variables {
                data_map.insert(key.clone(), serde_json::Value::String(value.clone()));
            }

            // DYNAMIC: Generate boolean helper variables automatically based on .conf metadata
            // This replaces all hardcoded style_is_*, with_tests_bool logic
            // Now any new template with new options will work automatically!
            Self::generate_boolean_helpers(
                &template_config.variables,
                &template_config.options_metadata,
                data_map,
            );
        }

        // Apply smart replacements
        let processed_content =
            Self::apply_smart_replacements(&template_content, name, &processed_names);

        // Render with handlebars for more advanced templating
        let rendered_content = handlebars
            .render_template(&processed_content, &data)
            .with_context(|| "Template rendering failed")?;

        // Process output filename with smart replacements
        let output_filename = output_file
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| Self::apply_smart_filename_replacements(n, name, &processed_names))
            .context("Invalid output filename")?;

        let final_output_path = output_file
            .parent()
            .context("Invalid output path")?
            .join(output_filename);

        // Create parent directories if needed
        if let Some(parent) = final_output_path.parent() {
            fs::create_dir_all(parent).await.with_context(|| {
                format!("Could not create parent directory: {}", parent.display())
            })?;
        }

        // Write processed content
        fs::write(&final_output_path, rendered_content)
            .await
            .with_context(|| {
                format!(
                    "Could not write output file: {}",
                    final_output_path.display()
                )
            })?;

        Ok(())
    }

    // Smart name processing for different patterns
    fn process_smart_names(name: &str) -> SmartNames {
        let name_lower = name.to_lowercase();

        // Hook name processing
        let hook_name = if name_lower.starts_with("use") {
            name.to_string()
        } else {
            format!("use{}", to_pascal_case(name))
        };

        // Context name processing
        let context_name = if name_lower.ends_with("context") {
            name.to_string()
        } else {
            format!("{}Context", to_pascal_case(name))
        };

        // Provider name processing
        let provider_name = if name_lower.ends_with("provider") {
            name.to_string()
        } else {
            let base_name = if name_lower.ends_with("context") {
                // Remove "Context" suffix if present
                let without_context = &name[..name.len() - 7];
                to_pascal_case(without_context)
            } else {
                to_pascal_case(name)
            };
            format!("{}Provider", base_name)
        };

        // Page name processing
        let page_name = if name_lower.ends_with("page") {
            name.to_string()
        } else {
            format!("{}Page", to_pascal_case(name))
        };

        SmartNames {
            hook_name,
            context_name,
            provider_name,
            page_name,
        }
    }

    // Apply smart content replacements
    fn apply_smart_replacements(content: &str, name: &str, smart_names: &SmartNames) -> String {
        let mut result = content.to_string();

        // Replace specific patterns with smart names
        result = result.replace("use$FILE_NAME", &smart_names.hook_name);
        result = result.replace("$FILE_NAMEContext", &smart_names.context_name);
        result = result.replace("$FILE_NAMEProvider", &smart_names.provider_name);
        result = result.replace("$FILE_NAMEPage", &smart_names.page_name);

        // Replace remaining $FILE_NAME with original name
        result = result.replace("$FILE_NAME", name);

        result
    }

    // Apply smart filename replacements
    fn apply_smart_filename_replacements(
        filename: &str,
        name: &str,
        smart_names: &SmartNames,
    ) -> String {
        let mut result = filename.to_string();

        // Replace specific patterns in filenames first
        result = result.replace("use$FILE_NAME", &smart_names.hook_name);
        result = result.replace("$FILE_NAMEContext", &smart_names.context_name);
        result = result.replace("$FILE_NAMEProvider", &smart_names.provider_name);
        result = result.replace("$FILE_NAMEPage", &smart_names.page_name);

        // Replace remaining $FILE_NAME with PascalCase name
        result = result.replace("$FILE_NAME", &to_pascal_case(name));

        result
    }

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
}

// Helper functions for naming conventions
fn pascal_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(value) = param.value().as_str() {
            out.write(&to_pascal_case(value))?;
        }
    }
    Ok(())
}

fn snake_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(value) = param.value().as_str() {
            out.write(&to_snake_case(value))?;
        }
    }
    Ok(())
}

fn kebab_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(value) = param.value().as_str() {
            out.write(&to_kebab_case(value))?;
        }
    }
    Ok(())
}

fn camel_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(value) = param.value().as_str() {
            out.write(&to_camel_case(value))?;
        }
    }
    Ok(())
}

fn upper_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(value) = param.value().as_str() {
            out.write(&value.to_uppercase())?;
        }
    }
    Ok(())
}

// Environment and utility helpers
fn timestamp_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let format = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("ISO");

    let now: DateTime<Utc> = Utc::now();
    let formatted = match format {
        "ISO" => now.to_rfc3339(),
        "date" => now.format("%Y-%m-%d").to_string(),
        "time" => now.format("%H:%M:%S").to_string(),
        "datetime" => now.format("%Y-%m-%d %H:%M:%S").to_string(),
        "unix" => now.timestamp().to_string(),
        _ => now.to_rfc3339(),
    };

    out.write(&formatted)?;
    Ok(())
}

fn uuid_helper(
    _h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let uuid = Uuid::new_v4();
    out.write(&uuid.to_string())?;
    Ok(())
}

fn env_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(var_name) = param.value().as_str() {
            let value = std::env::var(var_name).unwrap_or_default();
            out.write(&value)?;
        }
    }
    Ok(())
}

// Comparison helpers for conditional logic
fn eq_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param0 = h.param(0).map(|v| v.value());
    let param1 = h.param(1).map(|v| v.value());

    let result = match (param0, param1) {
        (Some(v1), Some(v2)) => v1 == v2,
        _ => false,
    };

    // For Handlebars conditionals, we write the boolean result
    out.write(&result.to_string())?;
    Ok(())
}

fn ne_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param0 = h.param(0).map(|v| v.value());
    let param1 = h.param(1).map(|v| v.value());

    let result = match (param0, param1) {
        (Some(v1), Some(v2)) => v1 != v2,
        _ => true,
    };

    // For Handlebars conditionals, we write the boolean result
    out.write(&result.to_string())?;
    Ok(())
}

// Utility functions for case conversions
fn to_pascal_case(s: &str) -> String {
    // If the string is already in PascalCase format, return as is
    if s.chars().next().is_some_and(|c| c.is_uppercase())
        && s.chars().all(|c| c.is_alphanumeric())
        && !s.contains('_')
        && !s.contains('-')
        && !s.contains(' ')
    {
        return s.to_string();
    }

    s.split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first
                    .to_uppercase()
                    .chain(chars.as_str().to_lowercase().chars())
                    .collect(),
            }
        })
        .collect()
}

fn to_camel_case(s: &str) -> String {
    let pascal = to_pascal_case(s);
    if pascal.is_empty() {
        return pascal;
    }

    let mut chars = pascal.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().chain(chars.as_str().chars()).collect(),
    }
}

fn to_snake_case(s: &str) -> String {
    s.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if c.is_uppercase() && i > 0 {
                vec!['_', c.to_lowercase().next().unwrap_or(c)]
            } else {
                vec![c.to_lowercase().next().unwrap_or(c)]
            }
        })
        .collect::<String>()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

fn to_kebab_case(s: &str) -> String {
    to_snake_case(s).replace('_', "-")
}
