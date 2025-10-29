//! Template inspection and description functionality
//!
//! This module provides functions for inspecting templates and displaying
//! detailed information about their configuration, variables, and usage.

use colored::*;

use super::config::{TemplateConfig, TemplateMetadata, VariableOption};

/// Print template header with name
pub fn print_template_header(name: &str, metadata: &TemplateMetadata) {
    println!("\n{} {}", "📋 Template:".bold(), name.cyan().bold());
    println!("{}", "=".repeat(50).cyan());
    println!();

    if !metadata.description.is_empty() {
        println!("{}", "Description:".bold());
        println!("  {}", metadata.description);
        println!();
    }
}

/// Print variables with metadata (required/enumerated variables)
pub fn print_required_variables(
    options: &std::collections::HashMap<String, VariableOption>,
    variables: &std::collections::HashMap<String, String>,
) {
    let mut sorted_metadata: Vec<_> = Vec::with_capacity(options.len());
    sorted_metadata.extend(options.iter());
    sorted_metadata.sort_by_key(|(name, _)| *name);

    for (var_name, metadata) in sorted_metadata {
        let default_value = variables.get(var_name).map(|s| s.as_str()).unwrap_or("");

        print!("  {} ", format!("--var {}=<value>", var_name).yellow());

        if !metadata.possible_values.is_empty() {
            println!();
            println!(
                "    {}: {}",
                "Options".bold(),
                metadata.possible_values.join(", ")
            );
        } else if metadata.var_type == "boolean" {
            println!();
            println!("    {}: boolean", "Type".bold());
        }

        if !default_value.is_empty() {
            println!("    {}: {}", "Default".bold(), default_value.green());
        }

        if !metadata.description.is_empty() {
            println!("    {}: {}", "Description".bold(), metadata.description);
        }

        println!();
    }
}

/// Print simple variables (optional variables without metadata)
pub fn print_optional_variables(
    variables: &std::collections::HashMap<String, String>,
    options: &std::collections::HashMap<String, VariableOption>,
) {
    // Pre-allocate assuming most variables might be optional
    let mut simple_vars: Vec<_> = Vec::with_capacity(variables.len());
    simple_vars.extend(
        variables
            .iter()
            .filter(|(name, _)| !options.contains_key(*name)),
    );
    simple_vars.sort_by_key(|(name, _)| *name);

    for (var_name, value) in simple_vars {
        println!("  {} ", format!("--var {}=<string>", var_name).yellow());
        println!("    {}: {}", "Default".bold(), value.green());
        println!();
    }
}

/// Print file filters showing which files will be generated
pub fn print_file_filters(filters: &std::collections::HashMap<String, String>) {
    println!("{}", "Files Generated:".bold().cyan());

    // Pre-allocate capacity for efficiency
    let capacity = filters.len() / 3 + 1; // Estimate equal distribution
    let mut always_files = Vec::with_capacity(capacity);
    let mut conditional_files = Vec::with_capacity(capacity);
    let mut default_files = Vec::with_capacity(capacity);

    for (filename, condition) in filters {
        let display_name = filename.replace("$FILE_NAME", "ComponentName");
        match condition.as_str() {
            "always" => always_files.push(display_name),
            "default" => default_files.push(display_name),
            _ => conditional_files.push((display_name, condition.clone())),
        }
    }

    always_files.sort();
    for file in always_files {
        println!("  {} {} {}", "✓".green(), file.bold(), "(always)".dimmed());
    }

    default_files.sort();
    for file in default_files {
        println!(
            "  {} {} {}",
            "○".yellow(),
            file.bold(),
            "(default)".dimmed()
        );
    }

    conditional_files.sort_by(|a, b| a.0.cmp(&b.0));
    for (file, condition) in conditional_files {
        let condition_display = format_condition(&condition);
        println!(
            "  {} {} {}",
            "○".yellow(),
            file.bold(),
            condition_display.dimmed()
        );
    }

    println!();
}

/// Print usage examples for the template
pub fn print_usage_examples(template_type: &str, config: &TemplateConfig) {
    println!("{}", "Usage Examples:".bold().magenta());
    println!();

    println!("  {} Basic (with defaults)", "#".dimmed());
    println!(
        "  {} ComponentName --type {}",
        "cli-frontend".cyan(),
        template_type
    );
    println!();

    let mut example_count = 0;
    const MAX_EXAMPLES: usize = 3;

    // Boolean examples
    for (var_name, metadata) in &config.options_metadata {
        if example_count >= MAX_EXAMPLES {
            break;
        }
        if metadata.var_type == "boolean" {
            let value = if config.variables.get(var_name).map(|s| s.as_str()) == Some("true") {
                "false"
            } else {
                "true"
            };
            println!("  {} With {}={}", "#".dimmed(), var_name, value);
            println!(
                "  {} ComponentName --type {} --var {}={}",
                "cli-frontend".cyan(),
                template_type,
                var_name,
                value
            );
            println!();
            example_count += 1;
        }
    }

    // Enum examples
    for (var_name, metadata) in &config.options_metadata {
        if example_count >= MAX_EXAMPLES {
            break;
        }
        if !metadata.possible_values.is_empty() && metadata.possible_values.len() > 1 {
            let current_value = config.variables.get(var_name).map(|s| s.as_str());
            let example_value = metadata
                .possible_values
                .iter()
                .find(|v| Some(v.as_str()) != current_value)
                .unwrap_or(&metadata.possible_values[0]);
            println!("  {} With {}={}", "#".dimmed(), var_name, example_value);
            println!(
                "  {} ComponentName --type {} --var {}={}",
                "cli-frontend".cyan(),
                template_type,
                var_name,
                example_value
            );
            println!();
            example_count += 1;
        }
    }

    // Full featured example
    if config.options_metadata.len() >= 2 {
        println!("  {} Full featured", "#".dimmed());
        print!(
            "  {} ComponentName --type {}",
            "cli-frontend".cyan(),
            template_type
        );
        let mut var_examples = Vec::with_capacity(3);
        for (var_name, metadata) in config.options_metadata.iter().take(3) {
            if !metadata.possible_values.is_empty() {
                var_examples.push(format!(
                    "--var {}={}",
                    var_name,
                    metadata.possible_values.first().unwrap()
                ));
            } else if metadata.var_type == "boolean" {
                var_examples.push(format!("--var {}=true", var_name));
            }
        }
        for example in var_examples {
            print!(" {}", example);
        }
        println!();
        println!();
    }

    println!();
}

/// Format a file condition for display
pub fn format_condition(condition: &str) -> String {
    let Some(without_prefix) = condition.strip_prefix("var_") else {
        return format!("({})", condition);
    };

    // Known multi-word values that should be kept together
    const MULTIWORD_SUFFIXES: &[(&str, &str)] = &[
        ("_styled_components", "styled-components"),
        ("_tests", "true"),
        ("_stories", "true"),
    ];

    // Check for known patterns
    for (suffix, value) in MULTIWORD_SUFFIXES {
        if let Some(var_name) = without_prefix.strip_suffix(suffix) {
            return format!("(--var {}={})", var_name, value);
        }
    }

    // Default: split by last underscore
    let parts: Vec<&str> = without_prefix.split('_').collect();
    if parts.len() == 1 {
        format!("(--var {}=true)", parts[0])
    } else {
        let var_name = parts[..parts.len() - 1].join("_");
        let value = parts[parts.len() - 1];
        format!("(--var {}={})", var_name, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_format_condition_always() {
        let result = format_condition("always");
        assert_eq!(result, "(always)");
    }

    #[test]
    fn test_format_condition_var_boolean() {
        let result = format_condition("var_with_tests");
        // "_tests" is a known multiword suffix, so it gets special treatment
        assert_eq!(result, "(--var with=true)");
    }

    #[test]
    fn test_format_condition_var_value() {
        let result = format_condition("var_style_scss");
        assert_eq!(result, "(--var style=scss)");
    }

    #[test]
    fn test_format_condition_multiword() {
        let result = format_condition("var_with_styled_components");
        assert_eq!(result, "(--var with=styled-components)");
    }

    #[test]
    fn test_format_condition_single_word() {
        let result = format_condition("var_typescript");
        assert_eq!(result, "(--var typescript=true)");
    }

    #[test]
    fn test_format_condition_no_var_prefix() {
        let result = format_condition("default");
        assert_eq!(result, "(default)");
    }

    #[test]
    fn test_print_template_header_basic() {
        let metadata = TemplateMetadata {
            name: "Test Template".to_string(),
            description: "".to_string(),
        };

        // Just verify it doesn't panic
        print_template_header("component", &metadata);
    }

    #[test]
    fn test_print_template_header_with_description() {
        let metadata = TemplateMetadata {
            name: "Component Template".to_string(),
            description: "React component with TypeScript".to_string(),
        };

        // Just verify it doesn't panic
        print_template_header("component", &metadata);
    }

    #[test]
    fn test_print_required_variables_with_options() {
        let mut options = HashMap::new();
        options.insert(
            "style".to_string(),
            VariableOption {
                var_type: "enum".to_string(),
                possible_values: vec!["scss".to_string(), "css".to_string()],
                description: "Style approach".to_string(),
            },
        );

        let variables = HashMap::new();

        // Just verify it doesn't panic
        print_required_variables(&options, &variables);
    }

    #[test]
    fn test_print_required_variables_boolean() {
        let mut options = HashMap::new();
        options.insert(
            "with_tests".to_string(),
            VariableOption {
                var_type: "boolean".to_string(),
                possible_values: vec![],
                description: "Include test files".to_string(),
            },
        );

        let variables = HashMap::new();

        // Just verify it doesn't panic
        print_required_variables(&options, &variables);
    }

    #[test]
    fn test_print_required_variables_with_defaults() {
        let mut options = HashMap::new();
        options.insert(
            "style".to_string(),
            VariableOption {
                var_type: "enum".to_string(),
                possible_values: vec!["scss".to_string(), "css".to_string()],
                description: "Style approach".to_string(),
            },
        );

        let mut variables = HashMap::new();
        variables.insert("style".to_string(), "scss".to_string());

        // Just verify it doesn't panic
        print_required_variables(&options, &variables);
    }

    #[test]
    fn test_print_optional_variables_empty() {
        let variables = HashMap::new();
        let options = HashMap::new();

        // Just verify it doesn't panic
        print_optional_variables(&variables, &options);
    }

    #[test]
    fn test_print_optional_variables_simple() {
        let mut variables = HashMap::new();
        variables.insert("author".to_string(), "John Doe".to_string());
        variables.insert("version".to_string(), "1.0.0".to_string());

        let options = HashMap::new();

        // Just verify it doesn't panic
        print_optional_variables(&variables, &options);
    }

    #[test]
    fn test_print_file_filters_always() {
        let mut filters = HashMap::new();
        filters.insert("$FILE_NAME.tsx".to_string(), "always".to_string());

        // Just verify it doesn't panic
        print_file_filters(&filters);
    }

    #[test]
    fn test_print_file_filters_conditional() {
        let mut filters = HashMap::new();
        filters.insert("$FILE_NAME.tsx".to_string(), "always".to_string());
        filters.insert(
            "$FILE_NAME.spec.tsx".to_string(),
            "var_with_tests".to_string(),
        );
        filters.insert(
            "$FILE_NAME.styles.scss".to_string(),
            "var_style_scss".to_string(),
        );

        // Just verify it doesn't panic
        print_file_filters(&filters);
    }

    #[test]
    fn test_print_file_filters_default() {
        let mut filters = HashMap::new();
        filters.insert("$FILE_NAME.tsx".to_string(), "default".to_string());

        // Just verify it doesn't panic
        print_file_filters(&filters);
    }

    #[test]
    fn test_print_usage_examples_basic() {
        let config = TemplateConfig::default();

        // Just verify it doesn't panic
        print_usage_examples("component", &config);
    }

    #[test]
    fn test_print_usage_examples_with_boolean() {
        let mut config = TemplateConfig::default();
        config.options_metadata.insert(
            "with_tests".to_string(),
            VariableOption {
                var_type: "boolean".to_string(),
                possible_values: vec![],
                description: "Include tests".to_string(),
            },
        );

        // Just verify it doesn't panic
        print_usage_examples("component", &config);
    }

    #[test]
    fn test_print_usage_examples_with_enum() {
        let mut config = TemplateConfig::default();
        config.options_metadata.insert(
            "style".to_string(),
            VariableOption {
                var_type: "enum".to_string(),
                possible_values: vec![
                    "scss".to_string(),
                    "css".to_string(),
                    "styled-components".to_string(),
                ],
                description: "Style approach".to_string(),
            },
        );

        // Just verify it doesn't panic
        print_usage_examples("component", &config);
    }

    #[test]
    fn test_format_condition_complex_multiword() {
        let result = format_condition("var_architecture_clean");
        assert_eq!(result, "(--var architecture=clean)");
    }

    #[test]
    fn test_format_condition_with_stories() {
        let result = format_condition("var_with_stories");
        // "_stories" is a known multiword suffix, so it gets special treatment
        assert_eq!(result, "(--var with=true)");
    }
}
