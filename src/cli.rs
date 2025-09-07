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
    
    /// Generate files without creating a folder
    #[arg(long = "no-folder")]
    pub no_folder: bool,
    
    /// Output directory for generated files (overrides config)
    #[arg(short = 'o', long = "output-dir")]
    pub output_dir: Option<PathBuf>,
    
    /// Path to custom configuration file
    #[arg(short = 'c', long = "config")]
    pub config: Option<PathBuf>,
    
    /// Show help and available templates
    #[arg(long = "help")]
    pub help: bool,
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
                        if !name.starts_with('.') {
                            templates.push(name.to_string());
                        }
                    }
                }
            }
        }
        
        templates.sort();
        templates
    }
    
    pub fn print_help() {
        println!("CLI Frontend Generator");
        println!("");
        println!("Usage: cli-frontend [name] [OPTIONS]");
        println!("");
        println!("Arguments:");
        println!("  <name>        The name of the template to generate");
        println!("");
        println!("Options:");
        println!("  -t, --type <TYPE>        The type of template to generate");
        println!("  --no-folder             Generate the file without creating a folder");
        println!("  -c, --config <CONFIG>   Path to custom configuration file");
        println!("  --help                  Show this help message");
        println!("");
        println!("Examples:");
        println!("  cli-frontend Button --type component");
        println!("  cli-frontend useAuth --type hook");
        println!("  cli-frontend ApiService --type service");
        println!("  cli-frontend UserContext --type context");
        println!("  cli-frontend HomePage --type page");
        println!("  cli-frontend UserStore --type store");
        println!("  cli-frontend Modal --type component --no-folder");
        println!("  cli-frontend Login --config ./my-config.conf");
    }
    
    /// Print help with dynamically discovered templates
    pub fn print_help_with_templates(templates_dir: &PathBuf) {
        let templates = Self::discover_templates(templates_dir);
        
        println!("CLI Frontend Generator");
        println!("");
        println!("Usage: cli-frontend [name] [OPTIONS]");
        println!("");
        println!("Arguments:");
        println!("  <name>        The name of the template to generate");
        println!("");
        println!("Options:");
        
        if templates.is_empty() {
            println!("  -t, --type <TYPE>        The type of template to generate");
        } else {
            println!("  -t, --type <TYPE>        The type of template to generate [possible values: {}]", templates.join(", "));
        }
        
        println!("  --no-folder             Generate the file without creating a folder");
        println!("  -o, --output-dir <DIR>  Output directory for generated files (overrides config)");
        println!("  -c, --config <CONFIG>   Path to custom configuration file");
        println!("  --help                  Show this help message");
        println!("");
        
        if !templates.is_empty() {
            println!("Available templates:");
            for template in &templates {
                println!("  - {}", template);
            }
            println!("");
        }
        
        println!("Examples:");
        if templates.contains(&"component".to_string()) {
            println!("  cli-frontend Button --type component");
        }
        if templates.contains(&"hook".to_string()) {
            println!("  cli-frontend useAuth --type hook");
        }
        if templates.contains(&"service".to_string()) {
            println!("  cli-frontend ApiService --type service");
        }
        if templates.contains(&"store".to_string()) {
            println!("  cli-frontend UserStore --type store");
        }
        
        println!("  cli-frontend Modal --type {} --no-folder", templates.first().unwrap_or(&"component".to_string()));
        println!("  cli-frontend MyTemplate --config ./my-config.conf");
    }
}