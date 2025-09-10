//! # Handlebars Helpers Module
//!
//! This module provides custom Handlebars helpers for the template engine.
//! Each helper is designed for specific code generation needs.

pub mod case_helpers;
pub mod comparison_helpers;
pub mod utility_helpers;

pub use case_helpers::*;
pub use comparison_helpers::*;
pub use utility_helpers::*;

use handlebars::Handlebars;

/// Register all custom helpers with a Handlebars instance
///
/// This function registers all available custom helpers for use in templates.
///
/// # Arguments
///
/// * `handlebars` - Mutable reference to Handlebars instance
pub fn register_all_helpers(handlebars: &mut Handlebars) {
    // Case conversion helpers
    handlebars.register_helper("pascal_case", Box::new(pascal_case_helper));
    handlebars.register_helper("snake_case", Box::new(snake_case_helper));
    handlebars.register_helper("kebab_case", Box::new(kebab_case_helper));
    handlebars.register_helper("camel_case", Box::new(camel_case_helper));
    handlebars.register_helper("upper_case", Box::new(upper_case_helper));

    // Utility helpers
    handlebars.register_helper("timestamp", Box::new(timestamp_helper));
    handlebars.register_helper("uuid", Box::new(uuid_helper));
    handlebars.register_helper("env", Box::new(env_helper));

    // Comparison helpers
    handlebars.register_helper("eq", Box::new(eq_helper));
    handlebars.register_helper("ne", Box::new(ne_helper));
}
