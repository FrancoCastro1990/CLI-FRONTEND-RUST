use anyhow::{bail, Result};
use std::fmt;

/// A validated template name
///
/// This newtype ensures that template names are valid and follow the required format:
/// - Non-empty
/// - Contains only alphanumeric characters, hyphens, and underscores
///
/// # Example
///
/// ```
/// use cli_frontend::types::TemplateName;
///
/// // Valid template names
/// let name = TemplateName::new("component").unwrap();
/// let name = TemplateName::new("my-template").unwrap();
/// let name = TemplateName::new("hook_v2").unwrap();
///
/// // Invalid template names (these will return errors)
/// assert!(TemplateName::new("").is_err());
/// assert!(TemplateName::new("my template").is_err());  // spaces not allowed
/// assert!(TemplateName::new("my/template").is_err());  // slashes not allowed
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateName(String);

impl TemplateName {
    /// Create a new template name with validation
    ///
    /// # Arguments
    ///
    /// * `name` - The template name to validate
    ///
    /// # Returns
    ///
    /// Returns `Ok(TemplateName)` if valid, otherwise returns an error.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Name is empty
    /// - Name contains invalid characters (only alphanumeric, hyphens, and underscores allowed)
    ///
    /// # Example
    ///
    /// ```
    /// use cli_frontend::types::TemplateName;
    ///
    /// let name = TemplateName::new("component")?;
    /// assert_eq!(name.as_str(), "component");
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn new(name: impl Into<String>) -> Result<Self> {
        let name = name.into();

        if name.is_empty() {
            bail!("Template name cannot be empty");
        }

        if !name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            bail!(
                "Template name '{}' contains invalid characters. \
                 Only alphanumeric characters, hyphens, and underscores are allowed.",
                name
            );
        }

        Ok(Self(name))
    }

    /// Get the inner string as a str
    ///
    /// # Example
    ///
    /// ```
    /// use cli_frontend::types::TemplateName;
    ///
    /// let name = TemplateName::new("component")?;
    /// assert_eq!(name.as_str(), "component");
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume self and return the inner String
    ///
    /// # Example
    ///
    /// ```
    /// use cli_frontend::types::TemplateName;
    ///
    /// let name = TemplateName::new("component")?;
    /// let string: String = name.into_string();
    /// assert_eq!(string, "component");
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for TemplateName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for TemplateName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<TemplateName> for String {
    fn from(name: TemplateName) -> Self {
        name.0
    }
}

/// Represents different types of templates that can be generated
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TemplateType {
    Component,
    Hook,
    Service,
    Context,
    Page,
    Feature,
    Store,
    Custom(String),
}

impl TemplateType {
    /// Parse a string into a TemplateType
    #[allow(dead_code)]
    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "component" => TemplateType::Component,
            "hook" => TemplateType::Hook,
            "service" => TemplateType::Service,
            "context" => TemplateType::Context,
            "page" => TemplateType::Page,
            "feature" => TemplateType::Feature,
            "store" => TemplateType::Store,
            _ => TemplateType::Custom(s.to_string()),
        }
    }

    /// Convert TemplateType to string slice
    #[allow(dead_code)]
    pub fn as_string(&self) -> &str {
        match self {
            TemplateType::Component => "component",
            TemplateType::Hook => "hook",
            TemplateType::Service => "service",
            TemplateType::Context => "context",
            TemplateType::Page => "page",
            TemplateType::Feature => "feature",
            TemplateType::Store => "store",
            TemplateType::Custom(s) => s.as_str(),
        }
    }
}

impl fmt::Display for TemplateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_name_valid() {
        assert!(TemplateName::new("component").is_ok());
        assert!(TemplateName::new("my-template").is_ok());
        assert!(TemplateName::new("hook_v2").is_ok());
        assert!(TemplateName::new("Component123").is_ok());
        assert!(TemplateName::new("test-template_v2").is_ok());
    }

    #[test]
    fn test_template_name_invalid() {
        assert!(TemplateName::new("").is_err());
        assert!(TemplateName::new("my template").is_err()); // spaces
        assert!(TemplateName::new("my/template").is_err()); // slashes
        assert!(TemplateName::new("my\\template").is_err()); // backslashes
        assert!(TemplateName::new("my.template").is_err()); // dots
        assert!(TemplateName::new("my@template").is_err()); // special chars
    }

    #[test]
    fn test_template_name_as_str() {
        let name = TemplateName::new("component").unwrap();
        assert_eq!(name.as_str(), "component");
    }

    #[test]
    fn test_template_name_display() {
        let name = TemplateName::new("component").unwrap();
        assert_eq!(format!("{}", name), "component");
    }

    #[test]
    fn test_template_name_as_ref() {
        let name = TemplateName::new("component").unwrap();
        let s: &str = name.as_ref();
        assert_eq!(s, "component");
    }

    #[test]
    fn test_template_name_into_string() {
        let name = TemplateName::new("component").unwrap();
        let string: String = name.into_string();
        assert_eq!(string, "component");
    }

    #[test]
    fn test_template_name_from() {
        let name = TemplateName::new("component").unwrap();
        let string: String = name.into();
        assert_eq!(string, "component");
    }

    #[test]
    fn test_template_type_parse() {
        assert_eq!(TemplateType::parse("component"), TemplateType::Component);
        assert_eq!(TemplateType::parse("Component"), TemplateType::Component);
        assert_eq!(TemplateType::parse("COMPONENT"), TemplateType::Component);
        assert_eq!(TemplateType::parse("hook"), TemplateType::Hook);
        assert_eq!(TemplateType::parse("service"), TemplateType::Service);
        assert_eq!(TemplateType::parse("context"), TemplateType::Context);
        assert_eq!(TemplateType::parse("page"), TemplateType::Page);
        assert_eq!(TemplateType::parse("feature"), TemplateType::Feature);
        assert_eq!(TemplateType::parse("store"), TemplateType::Store);
        assert_eq!(
            TemplateType::parse("custom"),
            TemplateType::Custom("custom".to_string())
        );
    }

    #[test]
    fn test_template_type_as_string() {
        assert_eq!(TemplateType::Component.as_string(), "component");
        assert_eq!(TemplateType::Hook.as_string(), "hook");
        assert_eq!(TemplateType::Service.as_string(), "service");
        assert_eq!(TemplateType::Context.as_string(), "context");
        assert_eq!(TemplateType::Page.as_string(), "page");
        assert_eq!(TemplateType::Feature.as_string(), "feature");
        assert_eq!(TemplateType::Store.as_string(), "store");
        assert_eq!(
            TemplateType::Custom("mytype".to_string()).as_string(),
            "mytype"
        );
    }

    #[test]
    fn test_template_type_display() {
        assert_eq!(format!("{}", TemplateType::Component), "component");
        assert_eq!(format!("{}", TemplateType::Hook), "hook");
        assert_eq!(
            format!("{}", TemplateType::Custom("test".to_string())),
            "test"
        );
    }

    #[test]
    fn test_template_type_roundtrip() {
        let types = vec![
            "component",
            "hook",
            "service",
            "context",
            "page",
            "feature",
            "store",
        ];
        for type_str in types {
            let template_type = TemplateType::parse(type_str);
            assert_eq!(template_type.as_string(), type_str);
        }
    }
}
