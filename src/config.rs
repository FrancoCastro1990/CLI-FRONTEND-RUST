use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub default_type: String,
    pub create_folder: bool,
    pub enable_hooks: bool,
    pub templates_dir: PathBuf,
    pub output_dir: PathBuf,
    pub architectures_dir: PathBuf,
    pub default_architecture: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArchitectureConfig {
    pub name: String,
    pub description: String,
    pub benefits: Vec<String>,
    pub limitations: Vec<String>,
    pub structure: Vec<ArchitectureStructure>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArchitectureStructure {
    pub path: String,
    pub template: String,
    pub filename_pattern: String,
    pub description: String,
}

impl Default for Config {
    fn default() -> Self {
        // Try multiple locations for templates directory
        let templates_dir = Self::find_templates_directory();
        let architectures_dir = Self::find_architectures_directory();

        Self {
            default_type: "component".to_string(),
            create_folder: true,
            enable_hooks: true,
            templates_dir,
            output_dir: PathBuf::from("."),
            architectures_dir,
            default_architecture: "screaming-architecture".to_string(),
        }
    }
}

impl Config {
    /// Find templates directory in order of preference
    pub fn find_templates_directory() -> PathBuf {
        let mut search_paths = vec![
            PathBuf::from("./templates"),     // Current directory first
            PathBuf::from("./.cli-template"), // Hidden directory in current
        ];

        // Add home directory paths
        if let Some(home_dir) = dirs::home_dir() {
            let home_paths = vec![
                home_dir.join(".cli-template"), // User's home directory
                home_dir.join(".config/cli-frontend/templates"), // XDG config directory
            ];

            search_paths.extend(home_paths);

            // On Unix systems, also check system directories
            #[cfg(unix)]
            search_paths.extend(vec![
                PathBuf::from("/usr/local/share/cli-frontend/templates"),
                PathBuf::from("/usr/share/cli-frontend/templates"),
            ]);

            // On Windows, also check common installation locations
            #[cfg(windows)]
            search_paths.extend(vec![
                home_dir.join(".cli-template/templates"), // Windows installer location
                PathBuf::from("C:\\Program Files\\cli-frontend\\templates"),
                PathBuf::from("C:\\cli-frontend\\templates"),
            ]);
        }

        // Return first existing directory, or default to home/.cli-template
        for path in search_paths {
            if path.exists() && path.is_dir() {
                return path;
            }
        }

        // Fallback to home directory or current directory
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".cli-template")
    }

    /// Find architectures directory in order of preference
    pub fn find_architectures_directory() -> PathBuf {
        let mut search_paths = vec![
            PathBuf::from("./architectures"),      // Current directory first
            PathBuf::from("./.cli-architectures"), // Hidden directory in current
        ];

        // Add home directory paths
        if let Some(home_dir) = dirs::home_dir() {
            let home_paths = vec![
                home_dir.join(".cli-architectures"), // User's home directory
                home_dir.join(".config/cli-frontend/architectures"), // XDG config directory
            ];

            search_paths.extend(home_paths);

            // On Unix systems, also check system directories
            #[cfg(unix)]
            search_paths.extend(vec![
                PathBuf::from("/usr/local/share/cli-frontend/architectures"),
                PathBuf::from("/usr/share/cli-frontend/architectures"),
            ]);

            // On Windows, also check common installation locations
            #[cfg(windows)]
            search_paths.extend(vec![
                home_dir.join(".cli-template/architectures"), // Windows installer location
                PathBuf::from("C:\\Program Files\\cli-frontend\\architectures"),
                PathBuf::from("C:\\cli-frontend\\architectures"),
            ]);
        }

        // Return first existing directory, or default to ./architectures
        for path in search_paths {
            if path.exists() && path.is_dir() {
                return path;
            }
        }

        // Fallback to current directory
        PathBuf::from("./architectures")
    }
    pub async fn load(config_path: &Option<PathBuf>) -> Result<Self> {
        let config_file = match config_path {
            Some(path) => path.clone(),
            None => {
                // Try multiple locations for config file
                let locations = vec![
                    PathBuf::from(".cli-frontend.conf"),   // Current directory first
                    PathBuf::from("./.cli-frontend.conf"), // Explicit current directory
                ];

                let mut found_config = None;
                for location in locations {
                    if location.exists() {
                        found_config = Some(location);
                        break;
                    }
                }

                // If not found locally, try home directory
                if found_config.is_none() {
                    if let Some(home_dir) = dirs::home_dir() {
                        let home_config = home_dir.join(".cli-frontend.conf");
                        if home_config.exists() {
                            found_config = Some(home_config);
                        }
                    }
                }

                // Use found config or default to home directory config
                match found_config {
                    Some(config) => config,
                    None => {
                        let home_dir = dirs::home_dir().context("Could not find home directory")?;
                        home_dir.join(".cli-frontend.conf")
                    }
                }
            }
        };

        if !config_file.exists() {
            // Create default config if it doesn't exist
            let default_config = Self::default();
            if config_path.is_none() {
                default_config.save(&config_file).await?;
            }
            return Ok(default_config);
        }

        let content = fs::read_to_string(&config_file)
            .await
            .with_context(|| format!("Could not read config file: {:?}", config_file))?;

        Self::parse_ini(&content)
    }

    pub async fn save(&self, path: &PathBuf) -> Result<()> {
        let content = self.to_ini();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::write(path, content)
            .await
            .with_context(|| format!("Could not save config file: {:?}", path))?;

        Ok(())
    }

    fn parse_ini(content: &str) -> Result<Self> {
        let mut config = Self::default();

        for line in content.lines() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            // Parse key=value pairs
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"').trim_matches('\'');

                match key {
                    "default_type" => config.default_type = value.to_string(),
                    "create_folder" => config.create_folder = value.parse().unwrap_or(true),
                    "enable_hooks" => config.enable_hooks = value.parse().unwrap_or(true),
                    "templates_dir" => {
                        let path = if value.starts_with('~') {
                            let home_dir =
                                dirs::home_dir().context("Could not find home directory")?;
                            home_dir.join(value.strip_prefix("~/").unwrap_or(value))
                        } else {
                            PathBuf::from(value)
                        };
                        config.templates_dir = path;
                    }
                    "output_dir" => config.output_dir = PathBuf::from(value),
                    "architectures_dir" => {
                        let path = if value.starts_with('~') {
                            let home_dir =
                                dirs::home_dir().context("Could not find home directory")?;
                            home_dir.join(value.strip_prefix("~/").unwrap_or(value))
                        } else {
                            PathBuf::from(value)
                        };
                        config.architectures_dir = path;
                    }
                    "default_architecture" => config.default_architecture = value.to_string(),
                    _ => {} // Ignore unknown keys
                }
            }
        }

        Ok(config)
    }

    fn to_ini(&self) -> String {
        let templates_dir = self
            .templates_dir
            .canonicalize()
            .unwrap_or(self.templates_dir.clone());
        let output_dir = self
            .output_dir
            .canonicalize()
            .unwrap_or(self.output_dir.clone());
        let architectures_dir = self
            .architectures_dir
            .canonicalize()
            .unwrap_or(self.architectures_dir.clone());
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
            self.default_type,
            self.create_folder,
            self.enable_hooks,
            templates_dir.display(),
            output_dir.display(),
            architectures_dir.display(),
            self.default_architecture
        )
    }

    /// Load architecture configuration from JSON file
    pub async fn load_architecture(&self, architecture_name: &str) -> Result<ArchitectureConfig> {
        let filename = if architecture_name == "default" {
            "default.json".to_string()
        } else {
            format!("{}.json", architecture_name)
        };

        let architecture_path = self.architectures_dir.join(&filename);

        if !architecture_path.exists() {
            // Try to load default architecture if requested one doesn't exist
            let default_path = self.architectures_dir.join("default.json");
            if default_path.exists() {
                let content = fs::read_to_string(&default_path).await.with_context(|| {
                    format!(
                        "Could not read default architecture file: {:?}",
                        default_path
                    )
                })?;
                return Self::parse_architecture_json(&content);
            } else {
                return Err(anyhow::anyhow!(
                    "Architecture '{}' not found and no default architecture available. File: {:?}",
                    architecture_name,
                    architecture_path
                ));
            }
        }

        let content = fs::read_to_string(&architecture_path)
            .await
            .with_context(|| {
                format!("Could not read architecture file: {:?}", architecture_path)
            })?;

        Self::parse_architecture_json(&content)
    }

    /// Parse architecture JSON content
    fn parse_architecture_json(content: &str) -> Result<ArchitectureConfig> {
        serde_json::from_str(content).with_context(|| "Failed to parse architecture JSON")
    }

    /// List all available architectures
    #[allow(dead_code)]
    pub fn list_architectures(&self) -> Result<Vec<String>> {
        let mut architectures = Vec::new();

        if !self.architectures_dir.exists() {
            return Ok(architectures);
        }

        for entry in std::fs::read_dir(&self.architectures_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".json") && !name.starts_with('.') {
                        let arch_name = name.strip_suffix(".json").unwrap_or(name);
                        if arch_name != "default" {
                            // Skip default.json in listing
                            architectures.push(arch_name.to_string());
                        }
                    }
                }
            }
        }

        architectures.sort();
        Ok(architectures)
    }
}
