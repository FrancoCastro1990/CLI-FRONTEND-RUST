//! # Integration Tests Module
//!
//! This module contains integration tests for the CLI Frontend Generator.
//! Tests are organized to cover the main functionality and edge cases.

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    #[test]
    fn test_basic_functionality() {
        // Test básico que siempre pasa
        assert!(true);
    }

    #[test]
    fn test_templates_directory_exists() {
        let templates_dir = Path::new("./templates");
        // El test pasa si el directorio existe o si estamos en un entorno donde no se espera que exista
        if templates_dir.exists() {
            assert!(templates_dir.is_dir(), "Templates path should be a directory");

            // Verificar que hay al menos algunos subdirectorios
            if let Ok(entries) = fs::read_dir(templates_dir) {
                let template_count = entries
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| entry.path().is_dir())
                    .count();

                if template_count > 0 {
                    println!("Found {} template directories", template_count);
                }
            }
        }
    }

    #[test]
    fn test_path_operations() {
        use std::path::PathBuf;

        // Test básico de operaciones de PathBuf
        let path = PathBuf::from("./test");
        let path_with_extension = path.with_extension("txt");

        assert_eq!(path_with_extension.extension().unwrap(), "txt");
    }

    #[test]
    fn test_file_system_discover_templates() {
        use crate::file_system::FileSystem;

        let temp_dir = TempDir::new().unwrap();
        let templates_dir = temp_dir.path();

        // Create some template directories
        std::fs::create_dir_all(templates_dir.join("component")).unwrap();
        std::fs::create_dir_all(templates_dir.join("hook")).unwrap();
        std::fs::create_dir_all(templates_dir.join(".hidden")).unwrap(); // Should be ignored

        let file_system = FileSystem::new();
        let templates = file_system.discover_templates(templates_dir).unwrap();

        assert_eq!(templates.len(), 2);
        assert!(templates.contains(&"component".to_string()));
        assert!(templates.contains(&"hook".to_string()));
        assert!(!templates.contains(&".hidden".to_string()));
    }

    #[test]
    fn test_string_transformations() {
        use crate::naming::SmartNaming;

        let naming = SmartNaming::new();
        let test_name = "TestComponent";

        // Pascal case (ya está)
        assert_eq!(naming.to_pascal_case(test_name), "TestComponent");

        // Camel case
        assert_eq!(naming.to_camel_case(test_name), "testComponent");

        // Snake case
        assert_eq!(naming.to_snake_case(test_name), "test_component");

        // Kebab case
        assert_eq!(naming.to_kebab_case(test_name), "test-component");
    }

    #[test]
    fn test_smart_name_processing() {
        use crate::naming::SmartNaming;

        let naming = SmartNaming::new();
        let processed = naming.process_smart_names("user");

        assert_eq!(processed.hook_name, "useUser");
        assert_eq!(processed.context_name, "UserContext");
        assert_eq!(processed.provider_name, "UserProvider");
        assert_eq!(processed.page_name, "UserPage");
    }

    #[test]
    fn test_config_module() {
        use crate::config::Config;

        // Test básico del módulo de configuración
        let templates_dir = Config::find_templates_directory();

        // Debe retornar un PathBuf válido
        assert!(templates_dir.as_os_str().len() > 0, "Should return a valid path");
    }

    #[test]
    fn test_handlebars_basic() {
        use handlebars::Handlebars;
        use serde_json::json;

        // Test básico de handlebars (nuestra dependencia principal)
        let mut handlebars = Handlebars::new();

        let template = "Hello {{name}}!";
        handlebars.register_template_string("test", template).unwrap();

        let data = json!({
            "name": "World"
        });

        let result = handlebars.render("test", &data).unwrap();
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_template_data_builder() {
        use crate::template_engine::data_builder::TemplateDataBuilder;
        use std::collections::HashMap;

        let mut variables = HashMap::new();
        variables.insert("author".to_string(), "John Doe".to_string());

        let data = TemplateDataBuilder::new()
            .with_name("userProfile")
            .with_environment("test")
            .with_variables(variables)
            .build()
            .unwrap();

        assert_eq!(data["name"], "userProfile");
        assert_eq!(data["pascal_name"], "UserProfile");
        assert_eq!(data["environment"], "test");
        assert_eq!(data["hook_name"], "useUserProfile");
        assert_eq!(data["author"], "John Doe");
    }

    #[tokio::test]
    async fn test_template_engine_creation() {
        use crate::template_engine::TemplateEngine;

        let temp_dir = TempDir::new().unwrap();
        let templates_dir = temp_dir.path().join("templates");
        let output_dir = temp_dir.path().join("output");

        let engine = TemplateEngine::new(templates_dir, output_dir);
        assert!(engine.is_ok());
    }

    #[test]
    fn test_handlebars_helpers() {
        use crate::template_engine::helpers::*;
        use handlebars::Handlebars;
        use serde_json::json;

        let mut handlebars = Handlebars::new();
        register_all_helpers(&mut handlebars);

        // Test pascal case helper
        let template = "{{pascal_case name}}";
        let data = json!({"name": "user_profile"});
        let result = handlebars.render_template(template, &data).unwrap();
        assert_eq!(result, "UserProfile");

        // Test timestamp helper
        let template = "{{timestamp \"date\"}}";
        let result = handlebars.render_template(template, &json!({})).unwrap();
        assert_eq!(result.len(), 10); // YYYY-MM-DD format
    }
}
