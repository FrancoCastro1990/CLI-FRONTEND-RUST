use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "cli-frontend",
    version,
    about = "A powerful CLI tool for generating React components, hooks, services, and more",
    long_about = "CLI Frontend Generator - Create React components, hooks, services, contexts, and pages with TypeScript support, SCSS modules, and comprehensive testing templates."
)]
pub struct Args {
    /// Name of the component, hook, service, context, or page to generate
    pub name: Option<String>,

    /// Type of template to generate
    #[arg(short = 't', long = "type")]
    pub template_type: Option<String>,

    /// Architecture pattern to use for feature templates
    #[arg(short = 'a', long = "architecture")]
    pub architecture: Option<String>,

    /// Generate files without creating a folder
    #[arg(long = "no-folder")]
    pub no_folder: bool,

    /// Output directory for generated files (overrides config)
    #[arg(short = 'o', long = "output-dir")]
    pub output_dir: Option<PathBuf>,

    /// Path to custom configuration file
    #[arg(short = 'c', long = "config")]
    pub config: Option<PathBuf>,

    /// Show detailed help with templates and architectures
    #[arg(long = "list")]
    pub list: bool,

    /// Template variables in KEY=VALUE format (can be used multiple times)
    /// Example: --var style=scss --var with_tests=false
    #[arg(long = "var", value_name = "KEY=VALUE")]
    pub vars: Vec<String>,
}

impl Args {
    /// Discovers available templates from the templates directory
    pub fn discover_templates(templates_dir: &PathBuf) -> Vec<String> {
        let mut templates = Vec::new();

        if !templates_dir.exists() {
            return templates;
        }

        if let Ok(entries) = std::fs::read_dir(templates_dir) {
            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    if let Some(name) = entry.file_name().to_str() {
                        if !name.starts_with('.') && name != "architectures" {
                            templates.push(name.to_string());
                        }
                    }
                }
            }
        }

        // Add the special "feature" type which uses architecture configurations
        templates.push("feature".to_string());
        templates.sort();
        templates
    }

    /// Discovers available architectures from the architectures directory
    pub fn discover_architectures(architectures_dir: &PathBuf) -> Vec<String> {
        let mut architectures = Vec::new();
        if !architectures_dir.exists() {
            return architectures;
        }
        if let Ok(entries) = std::fs::read_dir(architectures_dir) {
            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
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
        }
        architectures.sort();
        architectures
    }

    /// Parse --var arguments into a HashMap
    /// Example: ["style=scss", "with_tests=false"] -> {"style": "scss", "with_tests": "false"}
    pub fn parse_vars(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();

        for var_arg in &self.vars {
            if let Some((key, value)) = var_arg.split_once('=') {
                vars.insert(key.trim().to_string(), value.trim().to_string());
            } else {
                eprintln!("Warning: Invalid --var format '{}', expected KEY=VALUE", var_arg);
            }
        }

        vars
    }

    /// Print simple list of available templates and architectures
    pub fn print_simple_list(templates_dir: &PathBuf, architectures_dir: &PathBuf) {
        let templates = Self::discover_templates(templates_dir);
        let architectures = Self::discover_architectures(architectures_dir);

        println!("📋 Available Templates:");
        if templates.is_empty() {
            println!("  No templates found");
        } else {
            for template in &templates {
                println!("  • {}", template);
            }
        }

        println!();
        println!("🏗️  Available Architectures:");
        if architectures.is_empty() {
            println!("  No architectures found");
        } else {
            for arch in &architectures {
                println!("  • {}", arch);
            }
        }

        println!();
        println!("💡 Usage: cli-frontend <name> --type <template> [--architecture <arch>]");
    }
}
