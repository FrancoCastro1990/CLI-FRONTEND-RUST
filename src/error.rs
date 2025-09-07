use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)] // Allow for future use
pub enum CliError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Template error: {0}")]
    Template(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Handlebars error: {0}")]
    Handlebars(#[from] handlebars::TemplateError),
    
    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),
}

#[allow(dead_code)] // Allow for future use
pub type Result<T> = std::result::Result<T, CliError>;