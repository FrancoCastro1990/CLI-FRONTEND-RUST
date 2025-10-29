//! Template configuration structures and metadata.
//!
//! This module defines the configuration structures used to customize
//! template generation, including variables, file filters, and metadata.
//!
//! # Overview
//!
//! Templates can be configured using `.conf` files in INI format. These files
//! define metadata, variables with types and defaults, and conditional file
//! generation rules.
//!
//! # Example .conf file
//!
//! ```ini
//! [metadata]
//! name=React Component
//! description=Functional component with TypeScript
//!
//! [options]
//! style=scss
//! style_options=scss,css,styled-components,none
//! with_tests=true
//! with_tests_type=boolean
//!
//! [files]
//! $FILE_NAME.tsx=always
//! $FILE_NAME.spec.tsx=var_with_tests
//! $FILE_NAME.module.scss=var_style_scss
//! ```

use std::collections::HashMap;

/// Configuration for template generation, loaded from .conf files.
///
/// This struct contains all the settings needed to generate code from a template,
/// including variable values, environment settings, and conditional file generation
/// rules.
///
/// # Fields
///
/// * `variables` - Key-value pairs for template variables
/// * `environment` - Current environment (development/production)
/// * `enable_timestamps` - Whether to include timestamp variables
/// * `enable_uuid` - Whether to generate UUID variables
/// * `file_filters` - Conditional file generation rules
/// * `metadata` - Template name and description
/// * `options_metadata` - Type information for variables
///
/// # Example
///
/// ```
/// # use cli_frontend::template_engine::TemplateConfig;
/// # use std::collections::HashMap;
/// let mut config = TemplateConfig::default();
/// config.variables.insert("style".to_string(), "scss".to_string());
/// config.variables.insert("with_tests".to_string(), "true".to_string());
/// ```
#[derive(Debug, Clone)]
pub struct TemplateConfig {
    pub variables: HashMap<String, String>,
    pub environment: String,
    pub enable_timestamps: bool,
    pub enable_uuid: bool,
    /// Maps filename pattern to condition (e.g., "$FILE_NAME.spec.tsx" -> "var_with_tests")
    pub file_filters: HashMap<String, String>,
    /// Template metadata
    pub metadata: TemplateMetadata,
    /// Metadata about each variable option (for dynamic boolean helper generation)
    pub options_metadata: HashMap<String, VariableOption>,
}

/// Metadata about a template (name and description).
///
/// Provides human-readable information about what a template does
/// and what it generates.
///
/// # Example
///
/// ```
/// # use cli_frontend::template_engine::config::TemplateMetadata;
/// let metadata = TemplateMetadata {
///     name: "React Component".to_string(),
///     description: "Functional component with TypeScript".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct TemplateMetadata {
    pub name: String,
    pub description: String,
}

/// Metadata about a variable option from the .conf file.
///
/// Describes the type, possible values, and description of a template variable.
/// This information is used for validation, documentation, and generating
/// dynamic boolean helpers.
///
/// # Fields
///
/// * `var_type` - Type of variable: "boolean", "string", "enum", etc.
/// * `possible_values` - Valid values for enum types (from `{var}_options` in .conf)
/// * `description` - Human-readable description of what the variable controls
///
/// # Example
///
/// ```
/// # use cli_frontend::template_engine::config::VariableOption;
/// let option = VariableOption {
///     var_type: "enum".to_string(),
///     possible_values: vec!["scss".to_string(), "css".to_string()],
///     description: "Styling approach for the component".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct VariableOption {
    /// Type of variable: "boolean", "string", "enum", etc.
    pub var_type: String,
    /// Possible values (from {var}_options in .conf)
    pub possible_values: Vec<String>,
    /// Description of the variable
    pub description: String,
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self {
            variables: HashMap::new(),
            environment: std::env::var("NODE_ENV").unwrap_or_else(|_| "development".to_string()),
            enable_timestamps: true,
            enable_uuid: true,
            file_filters: HashMap::new(),
            metadata: TemplateMetadata::default(),
            options_metadata: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_config_default() {
        let config = TemplateConfig::default();

        assert!(config.variables.is_empty());
        assert!(config.enable_timestamps);
        assert!(config.enable_uuid);
        assert!(config.file_filters.is_empty());
        assert!(config.options_metadata.is_empty());
        // environment should be "development" or actual NODE_ENV
        assert!(!config.environment.is_empty());
    }

    #[test]
    fn test_template_metadata_default() {
        let metadata = TemplateMetadata::default();

        assert_eq!(metadata.name, "");
        assert_eq!(metadata.description, "");
    }

    #[test]
    fn test_variable_option_creation() {
        let option = VariableOption {
            var_type: "boolean".to_string(),
            possible_values: vec!["true".to_string(), "false".to_string()],
            description: "Enable tests".to_string(),
        };

        assert_eq!(option.var_type, "boolean");
        assert_eq!(option.possible_values.len(), 2);
        assert_eq!(option.description, "Enable tests");
    }

    #[test]
    fn test_template_config_with_variables() {
        let mut config = TemplateConfig::default();
        config
            .variables
            .insert("name".to_string(), "TestComponent".to_string());
        config
            .variables
            .insert("author".to_string(), "John Doe".to_string());

        assert_eq!(config.variables.len(), 2);
        assert_eq!(config.variables.get("name").unwrap(), "TestComponent");
        assert_eq!(config.variables.get("author").unwrap(), "John Doe");
    }

    #[test]
    fn test_template_config_with_file_filters() {
        let mut config = TemplateConfig::default();
        config.file_filters.insert(
            "$FILE_NAME.spec.tsx".to_string(),
            "var_with_tests".to_string(),
        );
        config.file_filters.insert(
            "$FILE_NAME.styles.scss".to_string(),
            "var_style_scss".to_string(),
        );

        assert_eq!(config.file_filters.len(), 2);
        assert_eq!(
            config.file_filters.get("$FILE_NAME.spec.tsx").unwrap(),
            "var_with_tests"
        );
    }

    #[test]
    fn test_template_config_clone() {
        let mut config = TemplateConfig::default();
        config
            .variables
            .insert("test".to_string(), "value".to_string());

        let cloned = config.clone();
        assert_eq!(cloned.variables.get("test").unwrap(), "value");
    }

    #[test]
    fn test_variable_option_default() {
        let option = VariableOption::default();

        assert_eq!(option.var_type, "");
        assert!(option.possible_values.is_empty());
        assert_eq!(option.description, "");
    }

    #[test]
    fn test_template_metadata_with_values() {
        let metadata = TemplateMetadata {
            name: "Component Template".to_string(),
            description: "React component template with tests".to_string(),
        };

        assert_eq!(metadata.name, "Component Template");
        assert_eq!(metadata.description, "React component template with tests");
    }

    #[test]
    fn test_variable_option_enum_type() {
        let option = VariableOption {
            var_type: "enum".to_string(),
            possible_values: vec![
                "scss".to_string(),
                "css".to_string(),
                "styled-components".to_string(),
            ],
            description: "Styling approach".to_string(),
        };

        assert_eq!(option.var_type, "enum");
        assert_eq!(option.possible_values.len(), 3);
        assert!(option.possible_values.contains(&"scss".to_string()));
        assert!(option
            .possible_values
            .contains(&"styled-components".to_string()));
    }
}
