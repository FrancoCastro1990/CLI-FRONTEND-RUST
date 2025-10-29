//! Template rendering utilities.
//!
//! Handles the core rendering logic including reading templates,
//! creating template data, rendering with Handlebars, and writing output files.
//!
//! # Key Functions
//!
//! - `create_handlebars()` - Initialize Handlebars with all helpers
//! - `create_template_data()` - Build data context with all variables
//! - `render_template()` - Render template with Handlebars
//! - `read_template()` - Read template file contents
//! - `write_output()` - Write rendered content to file

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use handlebars::Handlebars;
use serde_json::json;
use std::path::{Path, PathBuf};
use tokio::fs;
use uuid::Uuid;

use super::config::{TemplateConfig, VariableOption};
use super::handlebars_renderer::HandlebarsRenderer;
use super::naming::{
    apply_smart_filename_replacements, process_smart_names, to_camel_case, to_kebab_case,
    to_pascal_case, to_snake_case, SmartNames,
};

/// Creates a Handlebars instance with all helpers registered.
///
/// This function is a convenience wrapper around `HandlebarsRenderer::new()`
/// for backward compatibility.
///
/// # Returns
///
/// A configured `Handlebars` instance ready for template rendering.
///
/// # Example
///
/// ```
/// use cli_frontend::template_engine::renderer::create_handlebars;
/// use serde_json::json;
///
/// let handlebars = create_handlebars();
/// let result = handlebars.render_template(
///     "{{pascal_case name}}",
///     &json!({"name": "hello_world"})
/// ).unwrap();
/// assert_eq!(result, "HelloWorld");
/// ```
pub fn create_handlebars() -> Handlebars<'static> {
    HandlebarsRenderer::create_handlebars_instance()
}

/// Check if a value is truthy
fn is_truthy(value: &str) -> bool {
    matches!(value.to_lowercase().as_str(), "true" | "yes" | "1")
}

/// Generate boolean helper variables dynamically based on options metadata
///
/// For each variable with `_options` in .conf, creates `{var}_is_{value}` boolean helpers.
/// For each variable with `type=boolean` in .conf, creates `{var}_bool` boolean helper.
///
/// # Examples
/// ```text
/// style=scss with style_options=scss,styled-components,css,none
/// → generates: style_is_scss=true, style_is_styled_components=false, etc.
///
/// with_tests=true with with_tests_type=boolean
/// → generates: with_tests_bool=true
/// ```
pub fn generate_boolean_helpers(
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
                let helper_name = format!("{}_bool", var_name);
                data_map.insert(helper_name, serde_json::Value::Bool(is_truthy(value)));
            }
        }
    }
}

/// Creates template data with all variables and helpers.
///
/// Builds a complete JSON object containing all variables that will be
/// available in the template, including:
/// - Name variations (pascal, camel, snake, kebab)
/// - Smart names (hook, context, provider, page)
/// - Timestamps and UUIDs
/// - User-defined variables from config
/// - Dynamic boolean helpers
///
/// # Arguments
///
/// * `name` - The base name for generation
/// * `config` - Template configuration with variables
///
/// # Returns
///
/// A `serde_json::Value` containing all template variables.
///
/// # Example
///
/// ```
/// use cli_frontend::template_engine::renderer::create_template_data;
/// use cli_frontend::template_engine::TemplateConfig;
///
/// let config = TemplateConfig::default();
/// let data = create_template_data("MyComponent", &config);
///
/// assert_eq!(data["pascal_name"], "MyComponent");
/// assert_eq!(data["snake_name"], "my_component");
/// ```
pub fn create_template_data(name: &str, config: &TemplateConfig) -> serde_json::Value {
    let processed_names = process_smart_names(name);
    let now: DateTime<Utc> = Utc::now();
    let current_uuid = Uuid::new_v4();

    let mut data = json!({
        "name": name,
        "pascal_name": to_pascal_case(name).as_ref(),
        "snake_name": to_snake_case(name).as_ref(),
        "kebab_name": to_kebab_case(name).as_ref(),
        "camel_name": to_camel_case(name).as_ref(),
        "upper_name": name.to_uppercase(),
        "hook_name": processed_names.hook_name,
        "context_name": processed_names.context_name,
        "provider_name": processed_names.provider_name,
        "page_name": processed_names.page_name,
        "environment": config.environment,
        "timestamp": if config.enable_timestamps { now.to_rfc3339() } else { "".to_string() },
        "timestamp_iso": if config.enable_timestamps { now.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string() } else { "".to_string() },
        "date": if config.enable_timestamps { now.format("%Y-%m-%d").to_string() } else { "".to_string() },
        "time": if config.enable_timestamps { now.format("%H:%M:%S").to_string() } else { "".to_string() },
        "year": if config.enable_timestamps { now.format("%Y").to_string() } else { "".to_string() },
        "uuid": if config.enable_uuid { current_uuid.to_string() } else { "".to_string() },
        "uuid_simple": if config.enable_uuid { current_uuid.simple().to_string() } else { "".to_string() },
        "version": env!("CARGO_PKG_VERSION"),
        "generator_name": "CLI Frontend Generator",
        "generated": true
    });

    if let Some(data_map) = data.as_object_mut() {
        for (key, value) in &config.variables {
            data_map.insert(key.clone(), serde_json::Value::String(value.clone()));
        }
        generate_boolean_helpers(&config.variables, &config.options_metadata, data_map);
    }

    data
}

/// Render template with handlebars
pub fn render_template(
    handlebars: &Handlebars,
    content: &str,
    data: &serde_json::Value,
) -> Result<String> {
    handlebars
        .render_template(content, data)
        .with_context(|| "Template rendering failed")
}

/// Read template file content with optimized buffering
pub async fn read_template(path: &Path) -> Result<String> {
    use tokio::io::AsyncReadExt;

    let file = fs::File::open(path)
        .await
        .with_context(|| format!("Could not read template file: {}", path.display()))?;

    // Pre-allocate buffer based on file size
    let metadata = file
        .metadata()
        .await
        .with_context(|| format!("Could not get file metadata: {}", path.display()))?;

    let mut buffer = String::with_capacity(metadata.len() as usize);
    let mut reader = tokio::io::BufReader::new(file);

    reader
        .read_to_string(&mut buffer)
        .await
        .with_context(|| format!("Could not read template file: {}", path.display()))?;

    Ok(buffer)
}

/// Determine final output path with smart filename replacements
pub fn determine_output_path(
    base: &Path,
    name: &str,
    processed_names: &SmartNames,
) -> Result<PathBuf> {
    let output_filename = base
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| apply_smart_filename_replacements(n, name, processed_names))
        .context("Invalid output filename")?;

    Ok(base
        .parent()
        .context("Invalid output path")?
        .join(output_filename))
}

/// Write output file with content
pub async fn write_output(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .await
            .with_context(|| format!("Could not create parent directory: {}", parent.display()))?;
    }

    fs::write(path, content)
        .await
        .with_context(|| format!("Could not write output file: {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_read_template_success() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("test_template.txt");

        fs::write(&template_path, "Hello {{name}}!").await.unwrap();

        let content = read_template(&template_path).await.unwrap();
        assert_eq!(content, "Hello {{name}}!");
    }

    #[tokio::test]
    async fn test_read_template_not_found() {
        let result = read_template(Path::new("non_existent_file.txt")).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_write_output_creates_directories() {
        let temp_dir = TempDir::new().unwrap();
        let nested_path = temp_dir.path().join("nested").join("dir").join("file.txt");

        write_output(&nested_path, "test content").await.unwrap();

        let content = fs::read_to_string(&nested_path).await.unwrap();
        assert_eq!(content, "test content");
    }

    #[tokio::test]
    async fn test_write_output_success() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("output.txt");

        write_output(&output_path, "Hello World!").await.unwrap();

        let content = fs::read_to_string(&output_path).await.unwrap();
        assert_eq!(content, "Hello World!");
    }

    #[test]
    fn test_create_handlebars_has_helpers() {
        let handlebars = create_handlebars();

        // Verify helpers are registered by trying to use them
        let result = handlebars
            .render_template("{{pascal_case name}}", &json!({"name": "hello_world"}))
            .unwrap();
        assert_eq!(result, "HelloWorld");

        let result = handlebars
            .render_template("{{snake_case name}}", &json!({"name": "HelloWorld"}))
            .unwrap();
        assert_eq!(result, "hello_world");
    }

    #[test]
    fn test_create_template_data_basic() {
        let config = TemplateConfig::default();
        let data = create_template_data("MyComponent", &config);

        assert_eq!(data["name"], "MyComponent");
        assert_eq!(data["pascal_name"], "MyComponent");
        assert_eq!(data["snake_name"], "my_component");
        assert_eq!(data["kebab_name"], "my-component");
        assert_eq!(data["camel_name"], "myComponent");
        assert_eq!(data["upper_name"], "MYCOMPONENT");
        assert_eq!(data["generated"], true);
        assert!(data["timestamp"].as_str().is_some());
        assert!(data["uuid"].as_str().is_some());
    }

    #[test]
    fn test_create_template_data_with_variables() {
        let mut config = TemplateConfig::default();
        config
            .variables
            .insert("author".to_string(), "John Doe".to_string());
        config
            .variables
            .insert("version".to_string(), "1.0.0".to_string());

        let data = create_template_data("TestComponent", &config);

        assert_eq!(data["author"], "John Doe");
        assert_eq!(data["version"], "1.0.0");
        assert_eq!(data["name"], "TestComponent");
    }

    #[test]
    fn test_render_template_basic() {
        let handlebars = create_handlebars();
        let data = json!({"name": "World"});

        let result = render_template(&handlebars, "Hello {{name}}!", &data).unwrap();
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_render_template_with_helpers() {
        let handlebars = create_handlebars();
        let data = json!({"name": "hello_world"});

        let result = render_template(
            &handlebars,
            "{{pascal_case name}} - {{snake_case name}}",
            &data,
        )
        .unwrap();

        assert_eq!(result, "HelloWorld - hello_world");
    }

    #[test]
    fn test_determine_output_path_basic() {
        let base = Path::new("output/$FILE_NAME.tsx");
        let name = "MyComponent";
        let processed_names = process_smart_names(name);

        let result = determine_output_path(base, name, &processed_names).unwrap();
        assert_eq!(result.file_name().unwrap(), "MyComponent.tsx");
    }

    #[test]
    fn test_determine_output_path_with_replacements() {
        let base = Path::new("output/use$FILE_NAME.ts");
        let name = "MyHook";
        let processed_names = process_smart_names(name);

        let result = determine_output_path(base, name, &processed_names).unwrap();
        assert_eq!(result.file_name().unwrap(), "useMyHook.ts");
    }

    #[test]
    fn test_generate_boolean_helpers() {
        let mut variables = std::collections::HashMap::new();
        variables.insert("style".to_string(), "scss".to_string());
        variables.insert("with_tests".to_string(), "true".to_string());

        let mut options_metadata = std::collections::HashMap::new();
        options_metadata.insert(
            "style".to_string(),
            VariableOption {
                var_type: "enum".to_string(),
                possible_values: vec!["scss".to_string(), "css".to_string(), "none".to_string()],
                description: "Style approach".to_string(),
            },
        );
        options_metadata.insert(
            "with_tests".to_string(),
            VariableOption {
                var_type: "boolean".to_string(),
                possible_values: vec![],
                description: "Include tests".to_string(),
            },
        );

        let mut data_map = serde_json::Map::new();
        generate_boolean_helpers(&variables, &options_metadata, &mut data_map);

        // Check enum helpers
        assert_eq!(data_map["style_is_scss"], true);
        assert_eq!(data_map["style_is_css"], false);
        assert_eq!(data_map["style_is_none"], false);

        // Check boolean helper
        assert_eq!(data_map["with_tests_bool"], true);
    }

    #[test]
    fn test_is_truthy() {
        assert!(is_truthy("true"));
        assert!(is_truthy("TRUE"));
        assert!(is_truthy("yes"));
        assert!(is_truthy("YES"));
        assert!(is_truthy("1"));

        assert!(!is_truthy("false"));
        assert!(!is_truthy("no"));
        assert!(!is_truthy("0"));
        assert!(!is_truthy(""));
    }

    #[test]
    fn test_create_template_data_disabled_features() {
        let config = TemplateConfig {
            enable_timestamps: false,
            enable_uuid: false,
            ..Default::default()
        };

        let data = create_template_data("TestComponent", &config);

        assert_eq!(data["timestamp"], "");
        assert_eq!(data["uuid"], "");
        assert_eq!(data["name"], "TestComponent");
    }

    #[test]
    fn test_generate_boolean_helpers_styled_components() {
        let mut variables = std::collections::HashMap::new();
        variables.insert("style".to_string(), "styled-components".to_string());

        let mut options_metadata = std::collections::HashMap::new();
        options_metadata.insert(
            "style".to_string(),
            VariableOption {
                var_type: "enum".to_string(),
                possible_values: vec!["scss".to_string(), "styled-components".to_string()],
                description: "Style approach".to_string(),
            },
        );

        let mut data_map = serde_json::Map::new();
        generate_boolean_helpers(&variables, &options_metadata, &mut data_map);

        // Hyphen should be replaced with underscore in helper name
        assert_eq!(data_map["style_is_styled_components"], true);
        assert_eq!(data_map["style_is_scss"], false);
    }
}
