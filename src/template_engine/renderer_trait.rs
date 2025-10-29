//! Trait for template rendering abstraction.
//!
//! This module defines the `TemplateRenderer` trait which abstracts
//! the template rendering engine, allowing for different implementations
//! (Handlebars, Tera, etc.) and easier testing with mock renderers.

use anyhow::Result;
use serde_json::Value;

/// Trait for template rendering engines
///
/// Abstracts the rendering logic to allow for different implementations
/// and easier testing. Any template engine can implement this trait.
///
/// # Example
///
/// ```ignore
/// use cli_frontend::template_engine::TemplateRenderer;
///
/// struct MyCustomRenderer;
///
/// impl TemplateRenderer for MyCustomRenderer {
///     fn render(&self, template: &str, data: &serde_json::Value) -> anyhow::Result<String> {
///         // Custom rendering logic
///         Ok(format!("Rendered: {}", template))
///     }
/// }
/// ```
#[allow(dead_code)] // Public API trait
pub trait TemplateRenderer: Send + Sync {
    /// Render a template string with given data
    ///
    /// # Arguments
    ///
    /// * `template` - The template string to render
    /// * `data` - The JSON data context for template variables
    ///
    /// # Returns
    ///
    /// The rendered string or an error if rendering fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Template syntax is invalid
    /// - Required variables are missing
    /// - Rendering fails for any other reason
    fn render(&self, template: &str, data: &Value) -> Result<String>;
}
