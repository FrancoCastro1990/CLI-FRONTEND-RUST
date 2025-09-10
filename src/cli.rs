use crate::file_system::FileSystem;
use clap::Parser;
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
}

impl Args {
    /// Discovers available templates from the templates directory
    pub fn discover_templates(templates_dir: &PathBuf) -> Vec<String> {
        let file_system = FileSystem::new();
        file_system.discover_templates(templates_dir).unwrap_or_default()
    }

    /// Discovers available architectures from the architectures directory
    pub fn discover_architectures(architectures_dir: &PathBuf) -> Vec<String> {
        let file_system = FileSystem::new();
        file_system.discover_architectures(architectures_dir).unwrap_or_default()
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
