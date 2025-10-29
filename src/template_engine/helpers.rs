//! Handlebars helper functions for template rendering.
//!
//! Provides custom Handlebars helpers for case transformations, timestamps,
//! UUIDs, environment variables, and comparisons.
//!
//! # Available Helpers
//!
//! - **Case transformations**: `pascal_case`, `snake_case`, `kebab_case`, `camel_case`, `upper_case`
//! - **Timestamps**: `timestamp` with formats (ISO, date, time, datetime, unix)
//! - **UUID**: `uuid` for generating unique identifiers
//! - **Environment**: `env` for accessing environment variables
//! - **Comparisons**: `eq` (equals), `ne` (not equals)
//!
//! # Example
//!
//! ```
//! use handlebars::Handlebars;
//! use cli_frontend::template_engine::helpers::*;
//! use serde_json::json;
//!
//! let mut handlebars = Handlebars::new();
//! handlebars.register_helper("pascal_case", Box::new(pascal_case_helper));
//!
//! let result = handlebars.render_template(
//!     "{{pascal_case name}}",
//!     &json!({"name": "hello_world"})
//! ).unwrap();
//!
//! assert_eq!(result, "HelloWorld");
//! ```

use chrono::{DateTime, Utc};
use handlebars::{Handlebars, Helper, HelperResult, Output, RenderContext};
use uuid::Uuid;

use std::borrow::Cow;

use super::naming::{to_camel_case, to_kebab_case, to_pascal_case, to_snake_case};

/// Generic case transformation helper - DRY principle with Cow optimization
fn case_transform_helper<F>(h: &Helper, out: &mut dyn Output, transform: F) -> HelperResult
where
    F: Fn(&str) -> Cow<'_, str>,
{
    if let Some(param) = h.param(0) {
        if let Some(value) = param.value().as_str() {
            out.write(&transform(value))?;
        }
    }
    Ok(())
}

/// Handlebars helper for PascalCase transformation.
///
/// Converts the first parameter to PascalCase format.
///
/// # Template Usage
///
/// ```handlebars
/// {{pascal_case "hello_world"}}  -> HelloWorld
/// {{pascal_case name}}           -> Uses the 'name' variable
/// ```
pub fn pascal_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    case_transform_helper(h, out, to_pascal_case)
}

/// Handlebars helper for snake_case transformation
pub fn snake_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    case_transform_helper(h, out, to_snake_case)
}

/// Handlebars helper for kebab-case transformation
pub fn kebab_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    case_transform_helper(h, out, to_kebab_case)
}

/// Handlebars helper for camelCase transformation
pub fn camel_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    case_transform_helper(h, out, to_camel_case)
}

/// Handlebars helper for UPPER_CASE transformation
pub fn upper_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    case_transform_helper(h, out, |s: &str| Cow::Owned(s.to_uppercase()))
}

/// Handlebars helper for timestamp generation.
///
/// Generates timestamps in various formats based on the current time.
///
/// # Supported Formats
///
/// - `ISO` (default): ISO 8601 format (e.g., "2024-01-15T10:30:00Z")
/// - `date`: Date only (e.g., "2024-01-15")
/// - `time`: Time only (e.g., "10:30:00")
/// - `datetime`: Combined format (e.g., "2024-01-15 10:30:00")
/// - `unix`: Unix timestamp in seconds
///
/// # Template Usage
///
/// ```handlebars
/// {{timestamp}}              -> ISO format (default)
/// {{timestamp "date"}}       -> 2024-01-15
/// {{timestamp "time"}}       -> 10:30:00
/// {{timestamp "unix"}}       -> 1705315800
/// ```
pub fn timestamp_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let format = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("ISO");

    let now: DateTime<Utc> = Utc::now();
    let formatted = match format {
        "ISO" => now.to_rfc3339(),
        "date" => now.format("%Y-%m-%d").to_string(),
        "time" => now.format("%H:%M:%S").to_string(),
        "datetime" => now.format("%Y-%m-%d %H:%M:%S").to_string(),
        "unix" => now.timestamp().to_string(),
        _ => now.to_rfc3339(),
    };

    out.write(&formatted)?;
    Ok(())
}

/// Handlebars helper for UUID v4 generation.
///
/// Generates a random UUID v4 each time it's called.
///
/// # Template Usage
///
/// ```handlebars
/// {{uuid}}  -> e.g., "550e8400-e29b-41d4-a716-446655440000"
/// ```
pub fn uuid_helper(
    _h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let uuid = Uuid::new_v4();
    out.write(&uuid.to_string())?;
    Ok(())
}

/// Handlebars helper for environment variable access.
///
/// Reads an environment variable and returns its value.
/// Returns empty string if the variable doesn't exist.
///
/// # Template Usage
///
/// ```handlebars
/// {{env "NODE_ENV"}}     -> "production" or "development"
/// {{env "API_KEY"}}      -> Your API key or empty string
/// ```
pub fn env_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(var_name) = param.value().as_str() {
            let value = std::env::var(var_name).unwrap_or_default();
            out.write(&value)?;
        }
    }
    Ok(())
}

/// Handlebars helper for equality comparison.
///
/// Compares two values for equality. Useful for conditional rendering.
///
/// # Template Usage
///
/// ```handlebars
/// {{#if (eq style "scss")}}
///   Use SCSS styles
/// {{/if}}
/// ```
pub fn eq_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param0 = h.param(0).map(|v| v.value());
    let param1 = h.param(1).map(|v| v.value());

    let result = match (param0, param1) {
        (Some(v1), Some(v2)) => v1 == v2,
        _ => false,
    };

    // For Handlebars conditionals, we write the boolean result
    out.write(&result.to_string())?;
    Ok(())
}

/// Handlebars helper for inequality comparison.
///
/// Compares two values for inequality. Useful for conditional rendering.
///
/// # Template Usage
///
/// ```handlebars
/// {{#if (ne style "none")}}
///   Import styles
/// {{/if}}
/// ```
pub fn ne_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param0 = h.param(0).map(|v| v.value());
    let param1 = h.param(1).map(|v| v.value());

    let result = match (param0, param1) {
        (Some(v1), Some(v2)) => v1 != v2,
        _ => true,
    };

    // For Handlebars conditionals, we write the boolean result
    out.write(&result.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use handlebars::Handlebars;

    #[test]
    fn test_pascal_case_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("pascal_case", Box::new(pascal_case_helper));

        let result = handlebars
            .render_template(
                "{{pascal_case name}}",
                &serde_json::json!({"name": "hello_world"}),
            )
            .unwrap();

        assert_eq!(result, "HelloWorld");
    }

    #[test]
    fn test_snake_case_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("snake_case", Box::new(snake_case_helper));

        let result = handlebars
            .render_template(
                "{{snake_case name}}",
                &serde_json::json!({"name": "HelloWorld"}),
            )
            .unwrap();

        assert_eq!(result, "hello_world");
    }

    #[test]
    fn test_kebab_case_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("kebab-case", Box::new(kebab_case_helper));

        let result = handlebars
            .render_template(
                "{{kebab-case name}}",
                &serde_json::json!({"name": "HelloWorld"}),
            )
            .unwrap();

        assert_eq!(result, "hello-world");
    }

    #[test]
    fn test_camel_case_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("camel_case", Box::new(camel_case_helper));

        let result = handlebars
            .render_template(
                "{{camel_case name}}",
                &serde_json::json!({"name": "hello_world"}),
            )
            .unwrap();

        assert_eq!(result, "helloWorld");
    }

    #[test]
    fn test_upper_case_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("upper_case", Box::new(upper_case_helper));

        let result = handlebars
            .render_template(
                "{{upper_case name}}",
                &serde_json::json!({"name": "hello world"}),
            )
            .unwrap();

        assert_eq!(result, "HELLO WORLD");
    }

    #[test]
    fn test_timestamp_helper_iso() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("timestamp", Box::new(timestamp_helper));

        let result = handlebars
            .render_template("{{timestamp}}", &serde_json::json!({}))
            .unwrap();

        // Just verify it's not empty and has ISO format structure
        assert!(!result.is_empty());
        assert!(result.contains('T')); // ISO format has T separator
    }

    #[test]
    fn test_timestamp_helper_date() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("timestamp", Box::new(timestamp_helper));

        let result = handlebars
            .render_template("{{timestamp \"date\"}}", &serde_json::json!({}))
            .unwrap();

        // Format: YYYY-MM-DD
        assert!(result.len() == 10);
        assert!(result.contains('-'));
    }

    #[test]
    fn test_timestamp_helper_unix() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("timestamp", Box::new(timestamp_helper));

        let result = handlebars
            .render_template("{{timestamp \"unix\"}}", &serde_json::json!({}))
            .unwrap();

        // Unix timestamp should be parseable as integer
        assert!(result.parse::<i64>().is_ok());
    }

    #[test]
    fn test_uuid_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("uuid", Box::new(uuid_helper));

        let result = handlebars
            .render_template("{{uuid}}", &serde_json::json!({}))
            .unwrap();

        // UUID v4 format: xxxxxxxx-xxxx-4xxx-xxxx-xxxxxxxxxxxx
        assert_eq!(result.len(), 36);
        assert!(result.contains('-'));
        // Verify it's a valid UUID
        assert!(Uuid::parse_str(&result).is_ok());
    }

    #[test]
    fn test_env_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("env", Box::new(env_helper));

        // Set a test environment variable
        std::env::set_var("TEST_VAR", "test_value");

        let result = handlebars
            .render_template("{{env \"TEST_VAR\"}}", &serde_json::json!({}))
            .unwrap();

        assert_eq!(result, "test_value");

        // Clean up
        std::env::remove_var("TEST_VAR");
    }

    #[test]
    fn test_env_helper_not_found() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("env", Box::new(env_helper));

        let result = handlebars
            .render_template("{{env \"NON_EXISTENT_VAR\"}}", &serde_json::json!({}))
            .unwrap();

        // Should return empty string when variable doesn't exist
        assert_eq!(result, "");
    }

    #[test]
    fn test_eq_helper_true() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("eq", Box::new(eq_helper));

        let result = handlebars
            .render_template(
                "{{eq value1 value2}}",
                &serde_json::json!({"value1": "test", "value2": "test"}),
            )
            .unwrap();

        assert_eq!(result, "true");
    }

    #[test]
    fn test_eq_helper_false() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("eq", Box::new(eq_helper));

        let result = handlebars
            .render_template(
                "{{eq value1 value2}}",
                &serde_json::json!({"value1": "test", "value2": "other"}),
            )
            .unwrap();

        assert_eq!(result, "false");
    }

    #[test]
    fn test_ne_helper_true() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("ne", Box::new(ne_helper));

        let result = handlebars
            .render_template(
                "{{ne value1 value2}}",
                &serde_json::json!({"value1": "test", "value2": "other"}),
            )
            .unwrap();

        assert_eq!(result, "true");
    }

    #[test]
    fn test_ne_helper_false() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("ne", Box::new(ne_helper));

        let result = handlebars
            .render_template(
                "{{ne value1 value2}}",
                &serde_json::json!({"value1": "test", "value2": "test"}),
            )
            .unwrap();

        assert_eq!(result, "false");
    }

    #[test]
    fn test_case_helpers_with_empty_string() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("pascal_case", Box::new(pascal_case_helper));
        handlebars.register_helper("snake_case", Box::new(snake_case_helper));

        let result1 = handlebars
            .render_template("{{pascal_case name}}", &serde_json::json!({"name": ""}))
            .unwrap();

        let result2 = handlebars
            .render_template("{{snake_case name}}", &serde_json::json!({"name": ""}))
            .unwrap();

        assert_eq!(result1, "");
        assert_eq!(result2, "");
    }

    #[test]
    fn test_multiple_helpers_combined() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("pascal_case", Box::new(pascal_case_helper));
        handlebars.register_helper("snake_case", Box::new(snake_case_helper));

        let result = handlebars
            .render_template(
                "{{pascal_case name}} and {{snake_case name}}",
                &serde_json::json!({"name": "hello_world"}),
            )
            .unwrap();

        assert_eq!(result, "HelloWorld and hello_world");
    }
}
