//! # Case Conversion Helpers
//!
//! Handlebars helpers for converting strings between different case formats.
//! These helpers are commonly used in code generation templates.

use crate::naming::SmartNaming;
use handlebars::{Handlebars, Helper, HelperResult, Output, RenderContext};

/// Pascal case helper: {{pascal_case "user_profile"}} → "UserProfile"
pub fn pascal_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(value) = param.value().as_str() {
            let naming = SmartNaming::new();
            out.write(&naming.to_pascal_case(value))?;
        }
    }
    Ok(())
}

/// Snake case helper: {{snake_case "UserProfile"}} → "user_profile"
pub fn snake_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(value) = param.value().as_str() {
            let naming = SmartNaming::new();
            out.write(&naming.to_snake_case(value))?;
        }
    }
    Ok(())
}

/// Kebab case helper: {{kebab_case "UserProfile"}} → "user-profile"
pub fn kebab_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(value) = param.value().as_str() {
            let naming = SmartNaming::new();
            out.write(&naming.to_kebab_case(value))?;
        }
    }
    Ok(())
}

/// Camel case helper: {{camel_case "UserProfile"}} → "userProfile"
pub fn camel_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(value) = param.value().as_str() {
            let naming = SmartNaming::new();
            out.write(&naming.to_camel_case(value))?;
        }
    }
    Ok(())
}

/// Upper case helper: {{upper_case "hello"}} → "HELLO"
pub fn upper_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(value) = param.value().as_str() {
            out.write(&value.to_uppercase())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use handlebars::Handlebars;
    use serde_json::json;

    #[test]
    fn test_pascal_case_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("pascal_case", Box::new(pascal_case_helper));

        let template = "{{pascal_case name}}";
        let data = json!({"name": "user_profile"});
        let result = handlebars.render_template(template, &data).unwrap();

        assert_eq!(result, "UserProfile");
    }

    #[test]
    fn test_snake_case_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("snake_case", Box::new(snake_case_helper));

        let template = "{{snake_case name}}";
        let data = json!({"name": "UserProfile"});
        let result = handlebars.render_template(template, &data).unwrap();

        assert_eq!(result, "user_profile");
    }

    #[test]
    fn test_camel_case_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("camel_case", Box::new(camel_case_helper));

        let template = "{{camel_case name}}";
        let data = json!({"name": "UserProfile"});
        let result = handlebars.render_template(template, &data).unwrap();

        assert_eq!(result, "userProfile");
    }

    #[test]
    fn test_kebab_case_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("kebab_case", Box::new(kebab_case_helper));

        let template = "{{kebab_case name}}";
        let data = json!({"name": "UserProfile"});
        let result = handlebars.render_template(template, &data).unwrap();

        assert_eq!(result, "user-profile");
    }
}
