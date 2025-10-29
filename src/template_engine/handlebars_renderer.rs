//! Handlebars implementation of the TemplateRenderer trait.
//!
//! Provides a concrete implementation using the Handlebars template engine
//! with all custom helpers registered.

use anyhow::{Context, Result};
use handlebars::Handlebars;
use serde_json::Value;

use super::helpers::*;
use super::renderer_trait::TemplateRenderer;

/// Handlebars implementation of TemplateRenderer
///
/// Uses the Handlebars template engine with custom helpers for
/// case transformations, timestamps, UUIDs, and conditional rendering.
///
/// # Example
///
/// ```
/// use cli_frontend::template_engine::HandlebarsRenderer;
/// use cli_frontend::template_engine::TemplateRenderer;
/// use serde_json::json;
///
/// let renderer = HandlebarsRenderer::new();
/// let result = renderer.render(
///     "Hello {{name}}!",
///     &json!({"name": "World"})
/// ).unwrap();
/// assert_eq!(result, "Hello World!");
/// ```
pub struct HandlebarsRenderer {
    handlebars: Handlebars<'static>,
}

impl HandlebarsRenderer {
    /// Create a new HandlebarsRenderer with all helpers registered
    ///
    /// Initializes a Handlebars instance and registers all custom helpers:
    /// - Case transformations (pascal_case, snake_case, etc.)
    /// - Timestamps and UUIDs
    /// - Environment variables
    /// - Conditional helpers (eq, ne)
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();

        // Register all custom helpers
        handlebars.register_helper("pascal_case", Box::new(pascal_case_helper));
        handlebars.register_helper("snake_case", Box::new(snake_case_helper));
        handlebars.register_helper("kebab_case", Box::new(kebab_case_helper));
        handlebars.register_helper("camel_case", Box::new(camel_case_helper));
        handlebars.register_helper("upper_case", Box::new(upper_case_helper));
        handlebars.register_helper("timestamp", Box::new(timestamp_helper));
        handlebars.register_helper("uuid", Box::new(uuid_helper));
        handlebars.register_helper("env", Box::new(env_helper));
        handlebars.register_helper("eq", Box::new(eq_helper));
        handlebars.register_helper("ne", Box::new(ne_helper));

        Self { handlebars }
    }

    /// Get a reference to the inner Handlebars instance
    ///
    /// Useful for advanced use cases that need direct access to Handlebars
    #[allow(dead_code)] // Public API
    pub fn handlebars(&self) -> &Handlebars<'static> {
        &self.handlebars
    }

    /// Create a new Handlebars instance (for backward compatibility)
    ///
    /// Returns a fresh Handlebars instance with all helpers registered.
    /// This is used by the legacy `create_handlebars()` function.
    pub fn create_handlebars_instance() -> Handlebars<'static> {
        Self::new().handlebars
    }
}

impl Default for HandlebarsRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateRenderer for HandlebarsRenderer {
    fn render(&self, template: &str, data: &Value) -> Result<String> {
        self.handlebars
            .render_template(template, data)
            .context("Failed to render template with Handlebars")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_handlebars_renderer_basic() {
        let renderer = HandlebarsRenderer::new();
        let result = renderer
            .render("Hello {{name}}!", &json!({"name": "World"}))
            .unwrap();
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_handlebars_renderer_with_helper() {
        let renderer = HandlebarsRenderer::new();
        let result = renderer
            .render("{{pascal_case name}}", &json!({"name": "hello_world"}))
            .unwrap();
        assert_eq!(result, "HelloWorld");
    }

    #[test]
    fn test_handlebars_renderer_multiple_helpers() {
        let renderer = HandlebarsRenderer::new();
        let result = renderer
            .render(
                "{{pascal_case name}} - {{snake_case name}} - {{kebab_case name}}",
                &json!({"name": "HelloWorld"}),
            )
            .unwrap();
        assert_eq!(result, "HelloWorld - hello_world - hello-world");
    }

    #[test]
    fn test_handlebars_renderer_error_handling() {
        let renderer = HandlebarsRenderer::new();
        // Missing variable should still work (Handlebars renders as empty)
        let result = renderer.render("Hello {{missing}}!", &json!({})).unwrap();
        assert_eq!(result, "Hello !");
    }

    #[test]
    fn test_handlebars_renderer_default() {
        let renderer = HandlebarsRenderer::default();
        let result = renderer
            .render("Test {{value}}", &json!({"value": "123"}))
            .unwrap();
        assert_eq!(result, "Test 123");
    }
}
