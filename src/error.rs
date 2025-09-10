//! # Error Handling Module
//! 
//! This module provides comprehensive error types for the CLI Frontend Generator.
//! It follows Rust best practices for error handling with detailed context.

use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
#[allow(dead_code)] // Allow for future use
pub enum CliError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Template not found: {template} in {path}")]
    TemplateNotFound { 
        template: String, 
        path: PathBuf 
    },
    
    #[error("Invalid template data: {reason}")]
    InvalidTemplateData { 
        reason: String 
    },
    
    #[error("Template rendering failed: {source}")]
    TemplateRenderingFailed { 
        #[source] source: handlebars::TemplateError 
    },
    
    #[error("File operation failed: {operation} on {path}: {source}")]
    FileOperationFailed { 
        operation: String, 
        path: PathBuf, 
        #[source] source: std::io::Error 
    },
    
    #[error("Configuration error: {reason}")]
    Config { 
        reason: String 
    },
    
    #[error("Configuration file not found: {path}")]
    ConfigNotFound { 
        path: PathBuf 
    },
    
    #[error("Architecture '{architecture}' not found")]
    ArchitectureNotFound { 
        architecture: String 
    },
    
    #[error("Handlebars error: {0}")]
    Handlebars(#[from] handlebars::TemplateError),
    
    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Smart name processing failed: {reason}")]
    SmartNameProcessing { 
        reason: String 
    },
}

pub type Result<T> = std::result::Result<T, CliError>;