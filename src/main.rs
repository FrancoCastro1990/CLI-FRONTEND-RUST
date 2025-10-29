mod cli;
mod config;
mod template_engine;
mod types;
mod wizard;

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
        Args::print_simple_list(config.templates_dir(), config.architectures_dir());
        return Ok(());
    }

    // Handle --describe flag
    if let Some(template_name) = &args.describe {
        let template_engine =
            TemplateEngine::new(config.templates_dir().clone(), config.output_dir().clone())?;

        template_engine.describe_template(template_name).await?;
        return Ok(());
    }

    // Check if we should run wizard (no name and no template type provided)
    let final_args = if args.name.is_none() && args.template_type.is_none() {
        // Run interactive wizard
        let wizard_config = wizard::run_wizard(&config).await?;
        Args::from(wizard_config)
    } else {
        args
    };

    // Parse CLI variables first (before moving fields from final_args)
    let cli_vars = final_args.parse_vars();

    // Validate arguments (either from CLI or wizard)
    let name = final_args
        .name
        .ok_or_else(|| anyhow::anyhow!("No name was provided."))?;
    let template_type = match final_args.template_type {
        Some(t) => t,
        None => config.default_type().to_string(),
    };

    // Determine output directory (CLI arg overrides config)
    let output_dir = match final_args.output_dir {
        Some(dir) => dir,
        None => config.output_dir().clone(),
    };

    // Initialize template engine
    let template_engine = TemplateEngine::new(config.templates_dir().clone(), output_dir)?;

    let create_folder = !final_args.no_folder && config.create_folder();

    // Handle feature type specially
    if template_type == "feature" {
        let architecture = final_args
            .architecture
            .as_deref()
            .unwrap_or(config.default_architecture());

        println!(
            "{} Generating feature '{}' with {} architecture...",
            "🚀".bold(),
            name.bold(),
            architecture
        );

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

    template_engine
        .generate(&name, &template_type, create_folder, cli_vars)
        .await?;

    println!(
        "{} {} '{}' generated successfully!",
        "✅".green(),
        template_type,
        name.bold()
    );

    Ok(())
}
