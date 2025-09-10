//! # Template Engine Module
//!
//! This module provides the core templating functionality for the CLI Frontend Generator.
//! It supports Handlebars templating with custom helpers for code generation.
//!
//! ## Key Features
//!
//! - **Smart Name Processing**: Automatically generates appropriate names for different React patterns
//! - **Architecture-Aware Generation**: Supports multiple frontend architecture patterns
//! - **Custom Handlebars Helpers**: Case conversions, timestamps, UUIDs, environment variables
//! - **Async File Processing**: Concurrent template processing for better performance
//!
//! ## Architecture
//!
//! The template engine is split into focused modules:
//!
//! - `engine.rs`: Core template processing logic
//! - `helpers/`: Custom Handlebars helpers
//! - `data_builder.rs`: Template data construction with builder pattern
//!
//! ## Usage Example
//!
//! ```rust
//! use crate::template_engine::TemplateEngine;
//!
//! let engine = TemplateEngine::new(templates_dir, output_dir)?;
//! engine.generate("UserProfile", "component", true).await?;
//! ```

pub mod data_builder;
pub mod engine;
pub mod helpers;

pub use engine::TemplateEngine;
