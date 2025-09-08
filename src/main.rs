mod cli;
mod config;
mod template_engine;

#[cfg(test)]
mod tests;

use anyhow::Result;
use clap::Parser;
use cli::Args;
use colored::*;
use config::Config;
use template_engine::TemplateEngine;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Load configuration first to get templates directory
    let config = Config::load(&args.config).await?;

    if args.list {
        Args::print_help_with_templates(&config.templates_dir);
        return Ok(());
    }

    // Validate arguments
    let name = args
        .name
        .ok_or_else(|| anyhow::anyhow!("No name was provided."))?;
    let template_type = match args.template_type {
        Some(t) => t,
        None => config.default_type.clone(),
    };

    // Determine output directory (CLI arg overrides config)
    let output_dir = match args.output_dir {
        Some(dir) => dir,
        None => config.output_dir.clone(),
    };

    // Initialize template engine
    let template_engine = TemplateEngine::new(config.templates_dir.clone(), output_dir)?;

    // Handle feature type specially
    if template_type == "feature" {
        let architecture = args
            .architecture
            .as_deref()
            .unwrap_or(&config.default_architecture);

        println!(
            "{} Generating feature '{}' with {} architecture...",
            "🚀".bold(),
            name.bold(),
            architecture
        );

        let create_folder = !args.no_folder && config.create_folder;
        template_engine
            .generate_feature(&name, Some(architecture), create_folder, &config)
            .await?;

        println!(
            "{} Feature '{}' generated successfully with {} architecture!",
            "✅".green(),
            name.bold(),
            architecture
        );

        return Ok(());
    }

    // Validate template type exists
    if !template_engine.template_exists(&template_type) {
        eprintln!(
            "{} Unknown type '{}'. Available types:",
            "Error:".red(),
            template_type
        );
        for available in template_engine.list_templates()? {
            eprintln!("  - {}", available);
        }
        std::process::exit(1);
    }

    println!(
        "{} Generating {} '{}'...",
        "🚀".bold(),
        template_type,
        name.bold()
    );

    // Generate template
    let create_folder = !args.no_folder && config.create_folder;
    template_engine
        .generate(&name, &template_type, create_folder)
        .await?;

    println!(
        "{} {} '{}' generated successfully!",
        "✅".green(),
        template_type,
        name.bold()
    );

    Ok(())
}
