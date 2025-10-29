use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Helper function to expand tilde in paths
pub fn expand_path(value: &str) -> Result<PathBuf> {
    if value.starts_with('~') {
        let home_dir = dirs::home_dir().context("Could not find home directory")?;
        Ok(home_dir.join(value.strip_prefix("~/").unwrap_or(value)))
    } else {
        Ok(PathBuf::from(value))
    }
}

/// Parse INI-like configuration format
///
/// Returns a vector of (key, value) tuples
pub fn parse_ini(content: &str) -> Vec<(String, String)> {
    let mut pairs = Vec::new();

    for line in content.lines() {
        let line = line.trim();

        // Skip comments and empty lines
        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        // Parse key=value pairs
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let value = value
                .trim()
                .trim_matches('"')
                .trim_matches('\'')
                .to_string();
            pairs.push((key, value));
        }
    }

    pairs
}

/// Convert configuration to INI format string
pub fn to_ini(
    default_type: &str,
    create_folder: bool,
    enable_hooks: bool,
    templates_dir: &Path,
    output_dir: &Path,
    architectures_dir: &Path,
    default_architecture: &str,
) -> String {
    let templates_dir = templates_dir
        .canonicalize()
        .unwrap_or_else(|_| templates_dir.to_path_buf());
    let output_dir = output_dir
        .canonicalize()
        .unwrap_or_else(|_| output_dir.to_path_buf());
    let architectures_dir = architectures_dir
        .canonicalize()
        .unwrap_or_else(|_| architectures_dir.to_path_buf());

    format!(
        "# CLI Frontend Generator Configuration\n\
         # This file uses INI-like format for easy configuration\n\
         \n\
         # General settings\n\
         default_type={}\n\
         create_folder={}\n\
         enable_hooks={}\n\
         \n\
         # Paths configuration\n\
         templates_dir={}\n\
         output_dir={}\n\
         architectures_dir={}\n\
         \n\
         # Feature settings\n\
         default_architecture={}\n\
         \n\
         # Available template types are determined by the directories in templates_dir\n\
         # Available architectures are determined by JSON files in architectures_dir\n\
         # You can add new templates by creating new directories in templates_dir\n\
         # You can add new architectures by creating new JSON files in architectures_dir\n",
        default_type,
        create_folder,
        enable_hooks,
        templates_dir.display(),
        output_dir.display(),
        architectures_dir.display(),
        default_architecture
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ini_basic() {
        let content = r#"
# Comment
default_type=component
create_folder=true
templates_dir=/path/to/templates
"#;

        let pairs = parse_ini(content);
        assert_eq!(pairs.len(), 3);
        assert_eq!(
            pairs[0],
            ("default_type".to_string(), "component".to_string())
        );
        assert_eq!(pairs[1], ("create_folder".to_string(), "true".to_string()));
    }

    #[test]
    fn test_parse_ini_quoted_values() {
        let content = r#"
default_type="component"
templates_dir='/path/to/templates'
"#;

        let pairs = parse_ini(content);
        assert_eq!(pairs[0].1, "component");
        assert_eq!(pairs[1].1, "/path/to/templates");
    }

    #[test]
    fn test_expand_path_regular() {
        let path = expand_path("/usr/local/templates").unwrap();
        assert_eq!(path, PathBuf::from("/usr/local/templates"));
    }

    #[test]
    fn test_expand_path_tilde() {
        // This test depends on home directory being available
        if dirs::home_dir().is_some() {
            let path = expand_path("~/templates");
            assert!(path.is_ok());
            let path = path.unwrap();
            assert!(path.to_str().unwrap().contains("templates"));
            assert!(!path.to_str().unwrap().starts_with('~'));
        }
    }
}
