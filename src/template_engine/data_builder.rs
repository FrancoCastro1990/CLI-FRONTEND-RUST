//! # Template Data Builder
//!
//! Builder pattern for creating template data with all the necessary variables
//! for Handlebars template processing.

use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use std::collections::HashMap;
use uuid::Uuid;

use crate::naming::{ProcessedNames, SmartNaming};

/// Builder for creating template data with fluent API
#[derive(Debug, Default)]
pub struct TemplateDataBuilder {
    name: Option<String>,
    smart_names: Option<ProcessedNames>,
    environment: Option<String>,
    enable_timestamps: bool,
    enable_uuid: bool,
    custom_variables: HashMap<String, String>,
    naming: SmartNaming,
}

impl TemplateDataBuilder {
    /// Create a new TemplateDataBuilder
    pub fn new() -> Self {
        Self {
            enable_timestamps: true,
            enable_uuid: true,
            naming: SmartNaming::new(),
            ..Default::default()
        }
    }

    /// Set the base name for the template
    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        let name_str = name.into();
        self.smart_names = Some(self.naming.process_smart_names(&name_str));
        self.name = Some(name_str);
        self
    }

    /// Set the environment (development, production, etc.)
    pub fn with_environment<S: Into<String>>(mut self, environment: S) -> Self {
        self.environment = Some(environment.into());
        self
    }

    /// Enable or disable timestamp generation
    pub fn with_timestamps(mut self, enable: bool) -> Self {
        self.enable_timestamps = enable;
        self
    }

    /// Enable or disable UUID generation
    pub fn with_uuid(mut self, enable: bool) -> Self {
        self.enable_uuid = enable;
        self
    }

    /// Add multiple custom variables
    pub fn with_variables(mut self, variables: HashMap<String, String>) -> Self {
        self.custom_variables.extend(variables);
        self
    }

    /// Build the final template data as serde_json::Value
    pub fn build(self) -> Result<Value, TemplateDataError> {
        let name = self.name.ok_or(TemplateDataError::MissingName)?;
        let smart_names = self.smart_names.ok_or(TemplateDataError::MissingSmartNames)?;

        // Get current timestamp and UUID
        let now: DateTime<Utc> = Utc::now();
        let current_uuid = Uuid::new_v4();

        // Build base template data
        let mut data = json!({
            "name": name,
            "pascal_name": self.naming.to_pascal_case(&name),
            "snake_name": self.naming.to_snake_case(&name),
            "kebab_name": self.naming.to_kebab_case(&name),
            "camel_name": self.naming.to_camel_case(&name),
            "upper_name": name.to_uppercase(),
            // Smart processed names
            "hook_name": smart_names.hook_name,
            "context_name": smart_names.context_name,
            "provider_name": smart_names.provider_name,
            "page_name": smart_names.page_name,
            // Environment-aware variables
            "environment": self.environment.unwrap_or_else(|| "development".to_string()),
            "timestamp": if self.enable_timestamps { now.to_rfc3339() } else { "".to_string() },
            "timestamp_iso": if self.enable_timestamps { now.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string() } else { "".to_string() },
            "date": if self.enable_timestamps { now.format("%Y-%m-%d").to_string() } else { "".to_string() },
            "time": if self.enable_timestamps { now.format("%H:%M:%S").to_string() } else { "".to_string() },
            "year": if self.enable_timestamps { now.format("%Y").to_string() } else { "".to_string() },
            "uuid": if self.enable_uuid { current_uuid.to_string() } else { "".to_string() },
            "uuid_simple": if self.enable_uuid { current_uuid.simple().to_string() } else { "".to_string() },
            // Version info
            "version": env!("CARGO_PKG_VERSION"),
            "generator_name": "CLI Frontend Generator",
            "generated": true
        });

        // Add custom variables
        if let Some(data_map) = data.as_object_mut() {
            for (key, value) in &self.custom_variables {
                data_map.insert(key.clone(), Value::String(value.clone()));
            }
        }

        Ok(data)
    }
}

/// Errors that can occur when building template data
#[derive(Debug, thiserror::Error)]
pub enum TemplateDataError {
    #[error("Name is required for template data")]
    MissingName,

    #[error("Smart names processing failed")]
    MissingSmartNames,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_data_builder_basic() {
        let data = TemplateDataBuilder::new().with_name("userProfile").build().unwrap();

        assert_eq!(data["name"], "userProfile");
        assert_eq!(data["pascal_name"], "UserProfile");
        assert_eq!(data["camel_name"], "userProfile");
        assert_eq!(data["snake_name"], "user_profile");
        assert_eq!(data["kebab_name"], "user-profile");
        assert_eq!(data["hook_name"], "useUserProfile");
        assert_eq!(data["context_name"], "UserProfileContext");
    }

    #[test]
    fn test_template_data_builder_with_environment() {
        let data = TemplateDataBuilder::new()
            .with_name("test")
            .with_environment("production")
            .build()
            .unwrap();

        assert_eq!(data["environment"], "production");
    }

    #[test]
    fn test_template_data_builder_with_custom_variables() {
        let mut variables = HashMap::new();
        variables.insert("author".to_string(), "John Doe".to_string());
        variables.insert("license".to_string(), "MIT".to_string());

        let data =
            TemplateDataBuilder::new().with_name("test").with_variables(variables).build().unwrap();

        assert_eq!(data["author"], "John Doe");
        assert_eq!(data["license"], "MIT");
    }

    #[test]
    fn test_template_data_builder_timestamps_disabled() {
        let data =
            TemplateDataBuilder::new().with_name("test").with_timestamps(false).build().unwrap();

        assert_eq!(data["timestamp"], "");
        assert_eq!(data["date"], "");
    }

    #[test]
    fn test_template_data_builder_missing_name() {
        let result = TemplateDataBuilder::new().build();

        assert!(result.is_err());
        matches!(result.unwrap_err(), TemplateDataError::MissingName);
    }

    #[test]
    fn test_template_data_builder_uuid_disabled() {
        let data = TemplateDataBuilder::new().with_name("test").with_uuid(false).build().unwrap();

        assert_eq!(data["uuid"], "");
        assert_eq!(data["uuid_simple"], "");
    }
}
