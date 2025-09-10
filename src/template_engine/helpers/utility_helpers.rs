//! # Utility Helpers
//!
//! Handlebars helpers for timestamps, UUIDs, environment variables,
//! and other utility functions commonly needed in code generation.

use chrono::{DateTime, Utc};
use handlebars::{Handlebars, Helper, HelperResult, Output, RenderContext};
use uuid::Uuid;

/// Timestamp helper: {{timestamp "ISO"}} → "2025-09-09T10:30:00Z"
///
/// Supported formats:
/// - "ISO" (default): ISO 8601 format
/// - "date": YYYY-MM-DD
/// - "time": HH:MM:SS  
/// - "datetime": YYYY-MM-DD HH:MM:SS
/// - "unix": Unix timestamp
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
        "ISO" => now.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        "date" => now.format("%Y-%m-%d").to_string(),
        "time" => now.format("%H:%M:%S").to_string(),
        "datetime" => now.format("%Y-%m-%d %H:%M:%S").to_string(),
        "unix" => now.timestamp().to_string(),
        _ => now.to_rfc3339(),
    };

    out.write(&formatted)?;
    Ok(())
}

/// UUID helper: {{uuid}} → "550e8400-e29b-41d4-a716-446655440000"
///
/// Generates a random UUID v4
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

/// Environment variable helper: {{env "NODE_ENV"}} → "development"
///
/// Retrieves an environment variable value. Returns empty string if not found.
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

#[cfg(test)]
mod tests {
    use super::*;
    use handlebars::Handlebars;
    use serde_json::json;
    use std::env;

    #[test]
    fn test_timestamp_helper_iso() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("timestamp", Box::new(timestamp_helper));

        let template = "{{timestamp \"ISO\"}}";
        let result = handlebars.render_template(template, &json!({})).unwrap();

        // Should be a valid ISO timestamp
        assert!(result.contains("T"));
        assert!(result.ends_with("Z"));
    }

    #[test]
    fn test_timestamp_helper_date() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("timestamp", Box::new(timestamp_helper));

        let template = "{{timestamp \"date\"}}";
        let result = handlebars.render_template(template, &json!({})).unwrap();

        // Should be YYYY-MM-DD format
        assert_eq!(result.len(), 10);
        assert_eq!(result.chars().nth(4), Some('-'));
        assert_eq!(result.chars().nth(7), Some('-'));
    }

    #[test]
    fn test_uuid_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("uuid", Box::new(uuid_helper));

        let template = "{{uuid}}";
        let result = handlebars.render_template(template, &json!({})).unwrap();

        // Should be a valid UUID format
        assert_eq!(result.len(), 36);
        assert_eq!(result.chars().nth(8), Some('-'));
        assert_eq!(result.chars().nth(13), Some('-'));
    }

    #[test]
    fn test_env_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("env", Box::new(env_helper));

        // Set a test environment variable
        env::set_var("TEST_VAR", "test_value");

        let template = "{{env \"TEST_VAR\"}}";
        let result = handlebars.render_template(template, &json!({})).unwrap();

        assert_eq!(result, "test_value");

        // Clean up
        env::remove_var("TEST_VAR");
    }

    #[test]
    fn test_env_helper_missing() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("env", Box::new(env_helper));

        let template = "{{env \"NONEXISTENT_VAR\"}}";
        let result = handlebars.render_template(template, &json!({})).unwrap();

        assert_eq!(result, "");
    }
}
