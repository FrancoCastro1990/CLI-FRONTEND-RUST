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
    pub fn discover_architectures() -> Vec<String> {
        let mut architectures = Vec::new();
        let architectures_dir = PathBuf::from("./architectures");

        if !architectures_dir.exists() {
            return architectures;
        }

        if let Ok(entries) = std::fs::read_dir(&architectures_dir) {
            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.ends_with(".json") && !name.starts_with('.') {
                            let arch_name = name.strip_suffix(".json").unwrap_or(name);
                            if arch_name != "default" {
                                // Skip default.json in listing
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

    /// Print help with dynamically discovered templates
    pub fn print_help_with_templates(templates_dir: &PathBuf) {
        let templates = Self::discover_templates(templates_dir);
        let architectures = Self::discover_architectures();

        println!("CLI Frontend Generator");
        println!();
        println!("Usage: cli-frontend [name] [OPTIONS]");
        println!();
        println!("Arguments:");
        println!("  <name>        The name of the template to generate");
        println!();
        println!("Options:");

        if templates.is_empty() {
            println!("  -t, --type <TYPE>        The type of template to generate");
        } else {
            println!(
                "  -t, --type <TYPE>        The type of template to generate [possible values: {}]",
                templates.join(", ")
            );
        }

        if !architectures.is_empty() {
            println!(
                "  -a, --architecture <ARCH> Architecture pattern for feature templates [possible values: {}]",
                architectures.join(", ")
            );
        }

        println!("  --no-folder             Generate the file without creating a folder");
        println!(
            "  -o, --output-dir <DIR>  Output directory for generated files (overrides config)"
        );
        println!("  -c, --config <CONFIG>   Path to custom configuration file");
        println!("  --list                  Show detailed help with templates and architectures");
        println!("  -h, --help              Show basic help message");
        println!();

        if !templates.is_empty() {
            println!("Available templates:");
            for template in &templates {
                println!("  - {}", template);
            }
            println!();
        }

        if !architectures.is_empty() {
            println!("Available architectures (for feature type):");
            for arch in &architectures {
                println!("  - {}", arch);
            }
            println!();
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

        // Feature examples with architectures
        println!("  cli-frontend MyFeature --type feature");
        println!("  cli-frontend UserAuth --type feature --architecture mvc");
        println!("  cli-frontend ShoppingCart --type feature --architecture atomic-design");

        println!(
            "  cli-frontend Modal --type {} --no-folder",
            templates.first().unwrap_or(&"component".to_string())
        );
        println!("  cli-frontend MyTemplate --config ./my-config.conf");
    }
}
