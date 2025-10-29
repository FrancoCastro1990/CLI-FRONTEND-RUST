//! Template generation utilities
//!
//! This module contains helper functions for template generation including
//! validation, directory preparation, and variable merging.

use anyhow::{Context, Result};
use colored::*;
use std::path::{Path, PathBuf};
use tokio::fs;

use super::config::TemplateConfig;

/// Validate that template exists and return its directory
pub fn validate_template_exists(templates_dir: &Path, template_type: &str) -> Result<PathBuf> {
    let template_dir = templates_dir.join(template_type);
    if !template_dir.exists() {
        anyhow::bail!("Template directory not found: {}", template_dir.display());
    }
    Ok(template_dir)
}

/// Prepare output directory for generation
pub async fn prepare_output_directory(
    output_dir: &Path,
    name: &str,
    create_folder: bool,
) -> Result<PathBuf> {
    let output_path = if create_folder {
        output_dir.join(name)
    } else {
        output_dir.to_path_buf()
    };

    fs::create_dir_all(&output_path).await.with_context(|| {
        format!(
            "Could not create output directory: {}",
            output_path.display()
        )
    })?;

    Ok(output_path)
}

/// Merge CLI variables into template config and display them
pub fn merge_variables(
    cli_vars: std::collections::HashMap<String, String>,
    config: &mut TemplateConfig,
) {
    for (key, value) in cli_vars {
        config.variables.insert(key, value);
    }

    println!(
        "{} Using template config: environment={}",
        "⚙️".bold(),
        config.environment.blue()
    );

    if !config.variables.is_empty() {
        println!("{} Active variables:", "🔧".bold());
        for (key, value) in &config.variables {
            println!("  {} = {}", key.cyan(), value.green());
        }
    }
}

/// Check if a value is truthy
pub fn is_truthy(value: &str) -> bool {
    matches!(value.to_lowercase().as_str(), "true" | "yes" | "1")
}

/// Evaluate file condition to determine if a file should be generated
///
/// Supported conditions:
/// - "always" or "default" → always generate
/// - "var_X" → generate if variable X is truthy (true, yes, 1)
/// - "var_X_value" → generate if variable X equals "value"
///
/// # Examples
/// - "var_with_tests" → generate if with_tests=true
/// - "var_style_scss" → generate if style=scss
pub fn evaluate_file_condition(
    condition: &str,
    variables: &std::collections::HashMap<String, String>,
) -> bool {
    match condition.trim() {
        "always" | "default" => true,
        cond if cond.starts_with("var_") => {
            let var_part = cond.strip_prefix("var_").unwrap();

            // Boolean check: variable exists as-is
            if let Some(value) = variables.get(var_part) {
                return is_truthy(value);
            }

            // Value comparison: split by underscore
            if let Some(underscore_pos) = var_part.find('_') {
                let var_name = &var_part[..underscore_pos];
                let expected_value_raw = &var_part[underscore_pos + 1..];
                let expected_value = expected_value_raw.replace('_', "-");

                return variables
                    .get(var_name)
                    .is_some_and(|v| v == &expected_value || v == expected_value_raw);
            }

            false
        }
        _ => {
            eprintln!(
                "Warning: Unknown file condition '{}', skipping file",
                condition
            );
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_is_truthy() {
        assert!(is_truthy("true"));
        assert!(is_truthy("True"));
        assert!(is_truthy("TRUE"));
        assert!(is_truthy("yes"));
        assert!(is_truthy("1"));
        assert!(!is_truthy("false"));
        assert!(!is_truthy("no"));
        assert!(!is_truthy("0"));
    }

    #[test]
    fn test_evaluate_file_condition_always() {
        let variables = HashMap::new();
        assert!(evaluate_file_condition("always", &variables));
        assert!(evaluate_file_condition("default", &variables));
    }

    #[test]
    fn test_evaluate_file_condition_boolean() {
        let mut variables = HashMap::new();
        variables.insert("with_tests".to_string(), "true".to_string());
        assert!(evaluate_file_condition("var_with_tests", &variables));

        variables.insert("with_tests".to_string(), "false".to_string());
        assert!(!evaluate_file_condition("var_with_tests", &variables));
    }

    #[test]
    fn test_evaluate_file_condition_value_comparison() {
        let mut variables = HashMap::new();
        variables.insert("style".to_string(), "scss".to_string());
        assert!(evaluate_file_condition("var_style_scss", &variables));
        assert!(!evaluate_file_condition("var_style_css", &variables));
    }
}
