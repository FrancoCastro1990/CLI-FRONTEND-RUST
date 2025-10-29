//! Name transformation utilities for different naming conventions.
//!
//! Provides functions to convert strings between different case conventions
//! (PascalCase, camelCase, snake_case, kebab-case) and smart name processing
//! for React-specific patterns (hooks, contexts, providers, pages).
//!
//! # Example
//!
//! ```
//! use cli_frontend::template_engine::naming::{to_pascal_case, to_snake_case};
//!
//! assert_eq!(to_pascal_case("hello_world").as_ref(), "HelloWorld");
//! assert_eq!(to_snake_case("HelloWorld").as_ref(), "hello_world");
//! ```

use std::borrow::Cow;

/// Smart name variations for React-specific patterns.
///
/// This struct holds different name variations commonly used in React development,
/// such as hook names (useXxx), context names (XxxContext), provider names (XxxProvider),
/// and page names (XxxPage).
///
/// # Example
///
/// ```
/// use cli_frontend::template_engine::naming::process_smart_names;
///
/// let names = process_smart_names("Auth");
/// assert_eq!(names.hook_name, "useAuth");
/// assert_eq!(names.context_name, "AuthContext");
/// assert_eq!(names.provider_name, "AuthProvider");
/// assert_eq!(names.page_name, "AuthPage");
/// ```
#[derive(Debug)]
pub struct SmartNames {
    /// Hook name (e.g., "useAuth")
    pub hook_name: String,
    /// Context name (e.g., "AuthContext")
    pub context_name: String,
    /// Provider name (e.g., "AuthProvider")
    pub provider_name: String,
    /// Page name (e.g., "AuthPage")
    pub page_name: String,
}

/// Converts a string to PascalCase (also known as UpperCamelCase).
///
/// PascalCase capitalizes the first letter of each word and removes separators.
/// If the string is already in PascalCase format, returns a borrowed reference for zero-copy efficiency.
///
/// # Arguments
///
/// * `s` - The string to convert
///
/// # Returns
///
/// A `Cow<str>` that borrows the input if already in PascalCase, or owns a new String if conversion needed
///
/// # Example
///
/// ```
/// use cli_frontend::template_engine::naming::to_pascal_case;
///
/// assert_eq!(to_pascal_case("hello_world").as_ref(), "HelloWorld");
/// assert_eq!(to_pascal_case("hello-world").as_ref(), "HelloWorld");
/// assert_eq!(to_pascal_case("hello world").as_ref(), "HelloWorld");
/// assert_eq!(to_pascal_case("HelloWorld").as_ref(), "HelloWorld");
/// ```
#[inline]
pub fn to_pascal_case(s: &str) -> Cow<'_, str> {
    // If the string is already in PascalCase format, return borrowed
    if is_pascal_case(s) {
        return Cow::Borrowed(s);
    }

    // Otherwise, transform and return owned
    Cow::Owned(
        s.split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty())
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first
                        .to_uppercase()
                        .chain(chars.as_str().to_lowercase().chars())
                        .collect(),
                }
            })
            .collect(),
    )
}

/// Check if a string is already in PascalCase format
#[inline(always)]
fn is_pascal_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars = s.chars();
    let first = chars.next().unwrap();

    first.is_uppercase()
        && s.chars().all(|c| c.is_alphanumeric())
        && !s.contains('_')
        && !s.contains('-')
        && !s.contains(' ')
}

/// Converts a string to camelCase.
///
/// camelCase is like PascalCase but with the first letter lowercased.
/// Uses zero-copy optimization when possible.
///
/// # Arguments
///
/// * `s` - The string to convert
///
/// # Returns
///
/// A `Cow<str>` that borrows when no conversion needed, or owns a new String
///
/// # Example
///
/// ```
/// use cli_frontend::template_engine::naming::to_camel_case;
///
/// assert_eq!(to_camel_case("hello_world").as_ref(), "helloWorld");
/// assert_eq!(to_camel_case("HelloWorld").as_ref(), "helloWorld");
/// assert_eq!(to_camel_case("hello-world").as_ref(), "helloWorld");
/// ```
#[inline]
pub fn to_camel_case(s: &str) -> Cow<'_, str> {
    // Check if already in camelCase
    if is_camel_case(s) {
        return Cow::Borrowed(s);
    }

    let pascal = to_pascal_case(s);
    if pascal.is_empty() {
        return Cow::Owned(String::new());
    }

    let mut chars = pascal.chars();
    match chars.next() {
        None => Cow::Owned(String::new()),
        Some(first) => Cow::Owned(first.to_lowercase().chain(chars.as_str().chars()).collect()),
    }
}

/// Check if a string is already in camelCase format
#[inline(always)]
fn is_camel_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars = s.chars();
    let first = chars.next().unwrap();

    first.is_lowercase()
        && s.chars().all(|c| c.is_alphanumeric())
        && !s.contains('_')
        && !s.contains('-')
        && !s.contains(' ')
}

/// Converts a string to snake_case.
///
/// snake_case uses underscores to separate words, all lowercase.
/// Uses zero-copy optimization when possible.
///
/// # Arguments
///
/// * `s` - The string to convert
///
/// # Returns
///
/// A `Cow<str>` that borrows when no conversion needed, or owns a new String
///
/// # Example
///
/// ```
/// use cli_frontend::template_engine::naming::to_snake_case;
///
/// assert_eq!(to_snake_case("HelloWorld").as_ref(), "hello_world");
/// assert_eq!(to_snake_case("helloWorld").as_ref(), "hello_world");
/// assert_eq!(to_snake_case("hello-world").as_ref(), "hello_world");
/// ```
#[inline]
pub fn to_snake_case(s: &str) -> Cow<'_, str> {
    // Check if already in snake_case
    if is_snake_case(s) {
        return Cow::Borrowed(s);
    }

    Cow::Owned(
        s.chars()
            .enumerate()
            .flat_map(|(i, c)| {
                if c.is_uppercase() && i > 0 {
                    vec!['_', c.to_lowercase().next().unwrap_or(c)]
                } else {
                    vec![c.to_lowercase().next().unwrap_or(c)]
                }
            })
            .collect::<String>()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("_"),
    )
}

/// Check if a string is already in snake_case format
#[inline(always)]
fn is_snake_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    s.chars()
        .all(|c| c.is_lowercase() || c.is_numeric() || c == '_')
        && !s.contains('-')
        && !s.contains(' ')
        && s.chars().any(|c| c.is_alphabetic())
}

/// Converts a string to kebab-case (also known as dash-case).
///
/// kebab-case uses hyphens to separate words, all lowercase.
/// Uses zero-copy optimization when possible.
///
/// # Arguments
///
/// * `s` - The string to convert
///
/// # Returns
///
/// A `Cow<str>` that borrows when no conversion needed, or owns a new String
///
/// # Example
///
/// ```
/// use cli_frontend::template_engine::naming::to_kebab_case;
///
/// assert_eq!(to_kebab_case("HelloWorld").as_ref(), "hello-world");
/// assert_eq!(to_kebab_case("hello_world").as_ref(), "hello-world");
/// assert_eq!(to_kebab_case("helloWorld").as_ref(), "hello-world");
/// ```
#[inline]
pub fn to_kebab_case(s: &str) -> Cow<'_, str> {
    // Check if already in kebab-case
    if is_kebab_case(s) {
        return Cow::Borrowed(s);
    }

    let snake = to_snake_case(s);
    if snake.contains('_') {
        Cow::Owned(snake.replace('_', "-"))
    } else {
        snake
    }
}

/// Check if a string is already in kebab-case format
#[inline(always)]
fn is_kebab_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    s.chars()
        .all(|c| c.is_lowercase() || c.is_numeric() || c == '-')
        && !s.contains('_')
        && !s.contains(' ')
        && s.chars().any(|c| c.is_alphabetic())
}

/// Processes a name into smart names for React patterns.
///
/// Generates appropriate names for hooks (useX), contexts (XContext),
/// providers (XProvider), and pages (XPage) based on the input name.
/// Intelligently handles names that already have these suffixes.
///
/// # Arguments
///
/// * `name` - The base name to process
///
/// # Returns
///
/// A `SmartNames` struct containing all React-specific variations
///
/// # Example
///
/// ```
/// use cli_frontend::template_engine::naming::process_smart_names;
///
/// let names = process_smart_names("auth");
/// assert_eq!(names.hook_name, "useAuth");
/// assert_eq!(names.context_name, "AuthContext");
/// assert_eq!(names.provider_name, "AuthProvider");
/// assert_eq!(names.page_name, "AuthPage");
///
/// // Handles existing suffixes intelligently
/// let names = process_smart_names("useAuth");
/// assert_eq!(names.hook_name, "useAuth");  // No duplicate "use"
/// ```
pub fn process_smart_names(name: &str) -> SmartNames {
    let name_lower = name.to_lowercase();

    // Hook name processing
    let hook_name = if name_lower.starts_with("use") {
        name.to_string()
    } else {
        format!("use{}", to_pascal_case(name))
    };

    // Context name processing
    let context_name = if name_lower.ends_with("context") {
        name.to_string()
    } else {
        format!("{}Context", to_pascal_case(name))
    };

    // Provider name processing
    let provider_name = if name_lower.ends_with("provider") {
        name.to_string()
    } else {
        let base_name = if name_lower.ends_with("context") {
            // Remove "Context" suffix if present
            let without_context = &name[..name.len() - 7];
            to_pascal_case(without_context).into_owned()
        } else {
            to_pascal_case(name).into_owned()
        };
        format!("{}Provider", base_name)
    };

    // Page name processing
    let page_name = if name_lower.ends_with("page") {
        name.to_string()
    } else {
        format!("{}Page", to_pascal_case(name))
    };

    SmartNames {
        hook_name,
        context_name,
        provider_name,
        page_name,
    }
}

/// Applies smart content replacements for template content.
///
/// Replaces smart patterns like `use$FILE_NAME`, `$FILE_NAMEContext`, etc.
/// in template file contents with the appropriate React-specific names.
///
/// # Arguments
///
/// * `content` - The template content to process
/// * `name` - The original name
/// * `smart_names` - Pre-computed smart name variations
///
/// # Returns
///
/// A new String with all smart patterns replaced
///
/// # Example
///
/// ```
/// use cli_frontend::template_engine::naming::{apply_smart_replacements, process_smart_names};
///
/// let content = "export function use$FILE_NAME() { }";
/// let smart_names = process_smart_names("Auth");
/// let result = apply_smart_replacements(content, "Auth", &smart_names);
/// assert_eq!(result, "export function useAuth() { }");
/// ```
pub fn apply_smart_replacements(content: &str, name: &str, smart_names: &SmartNames) -> String {
    let mut result = content.to_string();

    // Replace specific patterns with smart names
    result = result.replace("use$FILE_NAME", &smart_names.hook_name);
    result = result.replace("$FILE_NAMEContext", &smart_names.context_name);
    result = result.replace("$FILE_NAMEProvider", &smart_names.provider_name);
    result = result.replace("$FILE_NAMEPage", &smart_names.page_name);

    // Replace remaining $FILE_NAME with original name
    result = result.replace("$FILE_NAME", name);

    result
}

/// Applies smart filename replacements.
///
/// Replaces patterns in filenames with appropriate smart names.
/// Converts filenames like `use$FILE_NAME.ts` to `useAuth.ts`.
///
/// # Arguments
///
/// * `filename` - The template filename pattern
/// * `name` - The original name
/// * `smart_names` - Pre-computed smart name variations
///
/// # Returns
///
/// A new String with the final filename
///
/// # Example
///
/// ```
/// use cli_frontend::template_engine::naming::{apply_smart_filename_replacements, process_smart_names};
///
/// let smart_names = process_smart_names("Auth");
/// let result = apply_smart_filename_replacements("use$FILE_NAME.ts", "Auth", &smart_names);
/// assert_eq!(result, "useAuth.ts");
/// ```
pub fn apply_smart_filename_replacements(
    filename: &str,
    name: &str,
    smart_names: &SmartNames,
) -> String {
    let mut result = filename.to_string();

    // Replace specific patterns in filenames first
    result = result.replace("use$FILE_NAME", &smart_names.hook_name);
    result = result.replace("$FILE_NAMEContext", &smart_names.context_name);
    result = result.replace("$FILE_NAMEProvider", &smart_names.provider_name);
    result = result.replace("$FILE_NAMEPage", &smart_names.page_name);

    // Replace remaining $FILE_NAME with PascalCase name
    result = result.replace("$FILE_NAME", &to_pascal_case(name));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
        assert_eq!(to_pascal_case("hello-world"), "HelloWorld");
        assert_eq!(to_pascal_case("HelloWorld"), "HelloWorld");
        assert_eq!(to_pascal_case("hello world"), "HelloWorld");
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("hello_world"), "helloWorld");
        assert_eq!(to_camel_case("HelloWorld"), "helloWorld");
        assert_eq!(to_camel_case("hello-world"), "helloWorld");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("helloWorld"), "hello_world");
        assert_eq!(to_snake_case("hello-world"), "hello_world");
    }

    #[test]
    fn test_to_kebab_case() {
        assert_eq!(to_kebab_case("HelloWorld"), "hello-world");
        assert_eq!(to_kebab_case("helloWorld"), "hello-world");
        assert_eq!(to_kebab_case("hello_world"), "hello-world");
    }

    #[test]
    fn test_process_smart_names() {
        let names = process_smart_names("auth");
        assert_eq!(names.hook_name, "useAuth");
        assert_eq!(names.context_name, "AuthContext");
        assert_eq!(names.provider_name, "AuthProvider");
        assert_eq!(names.page_name, "AuthPage");
    }
}
