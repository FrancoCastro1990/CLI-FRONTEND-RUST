//! CLI Frontend - React component and architecture generator
//!
//! This library provides template-based code generation for React applications,
//! including components, hooks, contexts, services, and various architectural patterns.

pub mod config;
pub mod template_engine;
pub mod types;

// Re-export commonly used types for convenience
pub use template_engine::{HandlebarsRenderer, TemplateConfig, TemplateRenderer};
