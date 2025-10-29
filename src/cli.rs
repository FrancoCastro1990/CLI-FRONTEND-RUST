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

    /// Show detailed information about a template including variables and file generation rules
    /// Example: --describe component
    #[arg(long = "describe", value_name = "TEMPLATE")]
    pub describe: Option<String>,
}

impl Args {
    /// Generic function to discover items in a directory
    fn discover_items<F>(dir: &PathBuf, filter: F) -> Vec<String>
    where
        F: Fn(&std::fs::DirEntry) -> Option<String>,
    {
        if !dir.exists() {
            return Vec::new();
        }

        let mut items: Vec<String> = std::fs::read_dir(dir)
            .ok()
            .into_iter()
            .flat_map(|entries| entries.flatten())
            .filter_map(|entry| filter(&entry))
            .collect();

        items.sort();
        items
    }

    /// Discovers available templates from the templates directory
    pub fn discover_templates(templates_dir: &PathBuf) -> Vec<String> {
        let mut templates = Self::discover_items(templates_dir, |entry| {
            if entry.file_type().ok()?.is_dir() {
                let name = entry.file_name().to_str()?.to_string();
                if !name.starts_with('.') && name != "architectures" {
                    return Some(name);
                }
            }
            None
        });

        // Add the special "feature" type which uses architecture configurations
        templates.push("feature".to_string());
        templates.sort();
        templates
    }

    /// Discovers available architectures from the architectures directory
    pub fn discover_architectures(architectures_dir: &PathBuf) -> Vec<String> {
        Self::discover_items(architectures_dir, |entry| {
            if entry.file_type().ok()?.is_file() {
                let name = entry.file_name().to_str()?.to_string();
                if name.ends_with(".json") && !name.starts_with('.') {
                    let arch_name = name.strip_suffix(".json")?;
                    if arch_name != "default" {
                        return Some(arch_name.to_string());
                    }
                }
            }
            None
        })
    }

    /// Parse --var arguments into a HashMap
    /// Example: ["style=scss", "with_tests=false"] -> {"style": "scss", "with_tests": "false"}
    pub fn parse_vars(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();

        for var_arg in &self.vars {
            if let Some((key, value)) = var_arg.split_once('=') {
                vars.insert(key.trim().to_string(), value.trim().to_string());
            } else {
                eprintln!(
                    "Warning: Invalid --var format '{}', expected KEY=VALUE",
                    var_arg
                );
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
