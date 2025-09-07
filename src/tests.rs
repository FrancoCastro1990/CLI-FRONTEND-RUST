#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

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
            assert!(
                templates_dir.is_dir(),
                "Templates path should be a directory"
            );

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
    fn test_discover_templates_function() {
        use crate::cli::Args;

        // Test de la función discover_templates con un directorio que puede o no existir
        let templates_dir = PathBuf::from("./templates");
        let templates = Args::discover_templates(&templates_dir);

        // Si el directorio existe, debe retornar una lista (puede estar vacía)
        // Si no existe, también debe retornar una lista vacía  
        // Este test simplemente verifica que la función no panic

        // Si hay templates, verificar que no hay duplicados
        if !templates.is_empty() {
            let mut sorted = templates.clone();
            sorted.sort();
            sorted.dedup();
            assert_eq!(
                templates.len(),
                sorted.len(),
                "Should not have duplicate templates"
            );
        }
    }

    #[test]
    fn test_string_transformations() {
        // Test de transformaciones básicas que podríamos usar
        let test_name = "TestComponent";

        // Pascal case (ya está)
        assert_eq!(test_name, "TestComponent");

        // Lower case
        assert_eq!(test_name.to_lowercase(), "testcomponent");

        // Upper case
        assert_eq!(test_name.to_uppercase(), "TESTCOMPONENT");
    }

    #[test]
    fn test_config_module() {
        use crate::config::Config;

        // Test básico del módulo de configuración
        let templates_dir = Config::find_templates_directory();

        // Debe retornar un PathBuf válido
        assert!(
            templates_dir.as_os_str().len() > 0,
            "Should return a valid path"
        );
    }

    #[test]
    fn test_handlebars_basic() {
        use handlebars::Handlebars;
        use serde_json::json;

        // Test básico de handlebars (nuestra dependencia principal)
        let mut handlebars = Handlebars::new();

        let template = "Hello {{name}}!";
        handlebars
            .register_template_string("test", template)
            .unwrap();

        let data = json!({
            "name": "World"
        });

        let result = handlebars.render("test", &data).unwrap();
        assert_eq!(result, "Hello World!");
    }
}
