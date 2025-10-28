use anyhow::Result;
use colored::*;
use inquire::{validator::Validation, Confirm, InquireError, Select, Text};
use std::path::PathBuf;

use crate::cli::Args;
use crate::config::Config;

/// Configuration captured from the interactive wizard
#[derive(Debug, Clone)]
pub struct WizardConfig {
    pub name: String,
    pub template_type: String,
    pub architecture: Option<String>,
    pub create_folder: bool,
    pub output_dir: Option<PathBuf>,
}

/// Types of generation available in the wizard
#[derive(Debug, Clone)]
enum GenerationType {
    Template,
    Feature,
}

impl GenerationType {
    fn as_display_string(&self) -> String {
        match self {
            GenerationType::Template => "📄 Template (component, hook, service, etc.)".to_string(),
            GenerationType::Feature => "🏗️  Complete Feature (with architecture)".to_string(),
        }
    }
}

/// Main wizard entry point
pub async fn run_wizard(config: &Config) -> Result<WizardConfig> {
    display_welcome();

    let generation_type = handle_prompt_result(prompt_generation_type())?;

    let wizard_config = match generation_type {
        GenerationType::Template => run_template_wizard(config)?,
        GenerationType::Feature => run_feature_wizard(config)?,
    };

    display_summary(&wizard_config);
    Ok(wizard_config)
}

/// Convert WizardConfig to Args for compatibility with existing code
impl From<WizardConfig> for Args {
    fn from(config: WizardConfig) -> Self {
        Args {
            name: Some(config.name),
            template_type: Some(config.template_type),
            architecture: config.architecture,
            no_folder: !config.create_folder,
            output_dir: config.output_dir,
            config: None,
            list: false,
            vars: Vec::new(), // Wizard doesn't support vars yet (could be added as future enhancement)
        }
    }
}

/// Display welcome message
fn display_welcome() {
    println!("{}", "🧙‍♂️ CLI Frontend Generator Wizard".bold().cyan());
    println!("{}", "=====================================".cyan());
    println!("Let's create something amazing! I'll guide you through the process.");
    println!("{}", "Press ESC at any time to cancel.".dimmed());
    println!();
}

/// Handle user cancellation gracefully
fn handle_cancellation() -> ! {
    println!("\n{}", "👋 Wizard canceled. See you next time!".yellow());
    std::process::exit(0);
}

/// Wrapper to handle InquireError::OperationCanceled gracefully
fn handle_prompt_result<T>(result: std::result::Result<T, InquireError>) -> Result<T> {
    match result {
        Ok(value) => Ok(value),
        Err(InquireError::OperationCanceled) => handle_cancellation(),
        Err(e) => Err(e.into()),
    }
}

/// Prompt user to select generation type (Template vs Feature)
fn prompt_generation_type() -> std::result::Result<GenerationType, InquireError> {
    let options = [GenerationType::Template, GenerationType::Feature];
    let display_options: Vec<String> = options.iter().map(|opt| opt.as_display_string()).collect();

    let selection = Select::new("What do you want to generate?", display_options).prompt()?;

    // Map display string back to enum
    if selection.contains("Template") {
        Ok(GenerationType::Template)
    } else {
        Ok(GenerationType::Feature)
    }
}

/// Run wizard flow for template generation
fn run_template_wizard(config: &Config) -> Result<WizardConfig> {
    // Get available templates
    let templates = Args::discover_templates(&config.templates_dir);

    if templates.is_empty() {
        return Err(anyhow::anyhow!("No templates found in templates directory"));
    }

    // Remove 'feature' from templates list as it's handled separately
    let template_options: Vec<String> = templates.into_iter().filter(|t| t != "feature").collect();

    let template_type =
        handle_prompt_result(Select::new("Select template type:", template_options).prompt())?;

    let name = prompt_name_with_suggestions(&template_type)?;
    let (create_folder, output_dir) = prompt_additional_options(config)?;

    Ok(WizardConfig {
        name,
        template_type,
        architecture: None,
        create_folder,
        output_dir,
    })
}

/// Run wizard flow for feature generation
fn run_feature_wizard(config: &Config) -> Result<WizardConfig> {
    // Get available architectures
    let architectures = Args::discover_architectures(&config.architectures_dir);

    if architectures.is_empty() {
        return Err(anyhow::anyhow!(
            "No architectures found in architectures directory"
        ));
    }

    let architecture =
        handle_prompt_result(Select::new("Select architecture pattern:", architectures).prompt())?;

    let name = prompt_name_with_suggestions("feature")?;
    let (create_folder, output_dir) = prompt_additional_options(config)?;

    Ok(WizardConfig {
        name,
        template_type: "feature".to_string(),
        architecture: Some(architecture),
        create_folder,
        output_dir,
    })
}

/// Prompt for name with context-aware suggestions and validation
fn prompt_name_with_suggestions(template_type: &str) -> Result<String> {
    let help_text = get_naming_help(template_type);

    let name = handle_prompt_result(
        Text::new(&format!("Enter the {} name:", template_type))
            .with_help_message(&help_text)
            .with_validator(|input: &str| {
                if input.trim().is_empty() {
                    Ok(Validation::Invalid("Name cannot be empty".into()))
                } else if input.trim().len() < 2 {
                    Ok(Validation::Invalid(
                        "Name must be at least 2 characters long".into(),
                    ))
                } else if !is_valid_name(input.trim()) {
                    Ok(Validation::Invalid(
                        "Name should contain only letters, numbers, and underscores".into(),
                    ))
                } else {
                    Ok(Validation::Valid)
                }
            })
            .prompt(),
    )?;

    Ok(name.trim().to_string())
}

/// Prompt for additional options (folder creation, output directory)
fn prompt_additional_options(config: &Config) -> Result<(bool, Option<PathBuf>)> {
    println!("\n{}", "Additional Options:".bold());

    let create_folder = handle_prompt_result(
        Confirm::new("Create in new folder?")
            .with_default(config.create_folder)
            .prompt(),
    )?;

    let use_custom_dir = handle_prompt_result(
        Confirm::new("Use custom output directory?")
            .with_default(false)
            .prompt(),
    )?;

    let output_dir = if use_custom_dir {
        let dir_input = handle_prompt_result(
            Text::new("Enter output directory path:")
                .with_default(&config.output_dir.to_string_lossy())
                .prompt(),
        )?;
        Some(PathBuf::from(dir_input))
    } else {
        None
    };

    Ok((create_folder, output_dir))
}

/// Get context-aware naming help for different template types
fn get_naming_help(template_type: &str) -> String {
    match template_type {
        "component" => "PascalCase (e.g., UserProfile, Navigation, ButtonGroup)",
        "hook" => "camelCase starting with 'use' (e.g., useAuth, useLocalStorage)",
        "service" => "PascalCase ending with 'Service' (e.g., ApiService, UserService)",
        "context" => "PascalCase ending with 'Context' (e.g., AuthContext, ThemeContext)",
        "page" => "PascalCase (e.g., HomePage, UserProfile, Dashboard)",
        "store" => "camelCase or PascalCase (e.g., userStore, AuthStore)",
        "feature" => "PascalCase for the feature name (e.g., UserManagement, Authentication)",
        _ => "Choose a descriptive name for your template",
    }
    .to_string()
}

/// Validate name format
fn is_valid_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}

/// Display summary of what will be generated
fn display_summary(config: &WizardConfig) {
    println!("\n{}", "📋 Summary:".bold().green());
    println!("  {} {}", "Name:".bold(), config.name);
    println!("  {} {}", "Type:".bold(), config.template_type);

    if let Some(arch) = &config.architecture {
        println!("  {} {}", "Architecture:".bold(), arch);
    }

    println!(
        "  {} {}",
        "Create folder:".bold(),
        if config.create_folder { "Yes" } else { "No" }
    );

    if let Some(dir) = &config.output_dir {
        println!("  {} {}", "Output directory:".bold(), dir.display());
    }

    println!("\n{}", "🚀 Generating...".bold().yellow());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_name() {
        assert!(is_valid_name("MyComponent"));
        assert!(is_valid_name("useAuth"));
        assert!(is_valid_name("user_profile"));
        assert!(is_valid_name("Component123"));
        assert!(!is_valid_name(""));
        assert!(!is_valid_name("my-component")); // hyphens not allowed
        assert!(!is_valid_name("my component")); // spaces not allowed
    }

    #[test]
    fn test_generation_type_display() {
        let template = GenerationType::Template;
        let feature = GenerationType::Feature;

        assert!(template.as_display_string().contains("Template"));
        assert!(feature.as_display_string().contains("Feature"));
    }
}
