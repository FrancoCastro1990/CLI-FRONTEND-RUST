use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::fs;

use super::parser::{expand_path, parse_ini, to_ini};
use super::Config;

impl Config {
    /// Generic function to find directory with customizable search paths
    pub fn find_directory(
        local_paths: Vec<PathBuf>,
        home_subpaths: Vec<&str>,
        system_paths: Vec<PathBuf>,
        fallback: PathBuf,
    ) -> PathBuf {
        let mut search_paths = local_paths;

        if let Some(home_dir) = dirs::home_dir() {
            for subpath in home_subpaths {
                search_paths.push(home_dir.join(subpath));
            }

            #[cfg(unix)]
            search_paths.extend(system_paths.clone());

            #[cfg(windows)]
            search_paths.extend(system_paths);
        }

        search_paths
            .into_iter()
            .find(|path| path.exists() && path.is_dir())
            .unwrap_or(fallback)
    }

    /// Find templates directory in order of preference
    pub fn find_templates_directory() -> PathBuf {
        let local_paths = vec![
            PathBuf::from("./templates"),
            PathBuf::from("./.cli-template"),
        ];

        let home_subpaths = vec![".cli-template", ".config/cli-frontend/templates"];

        let system_paths = vec![
            PathBuf::from("/usr/local/share/cli-frontend/templates"),
            PathBuf::from("/usr/share/cli-frontend/templates"),
            PathBuf::from("C:\\Program Files\\cli-frontend\\templates"),
            PathBuf::from("C:\\cli-frontend\\templates"),
        ];

        let fallback = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".cli-template");

        Self::find_directory(local_paths, home_subpaths, system_paths, fallback)
    }

    /// Find architectures directory in order of preference
    pub fn find_architectures_directory() -> PathBuf {
        let local_paths = vec![
            PathBuf::from("./architectures"),
            PathBuf::from("./.cli-architectures"),
        ];

        let home_subpaths = vec![".cli-architectures", ".config/cli-frontend/architectures"];

        let system_paths = vec![
            PathBuf::from("/usr/local/share/cli-frontend/architectures"),
            PathBuf::from("/usr/share/cli-frontend/architectures"),
            PathBuf::from("C:\\Program Files\\cli-frontend\\architectures"),
            PathBuf::from("C:\\cli-frontend\\architectures"),
        ];

        let fallback = PathBuf::from("./architectures");

        Self::find_directory(local_paths, home_subpaths, system_paths, fallback)
    }

    /// Load configuration from file or create default
    pub async fn load(config_path: &Option<PathBuf>) -> Result<Self> {
        let config_file: std::borrow::Cow<'_, Path> = match config_path {
            Some(path) => std::borrow::Cow::Borrowed(path.as_path()),
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
                let path = match found_config {
                    Some(config) => config,
                    None => {
                        let home_dir = dirs::home_dir().context("Could not find home directory")?;
                        home_dir.join(".cli-frontend.conf")
                    }
                };
                std::borrow::Cow::Owned(path)
            }
        };

        if !config_file.exists() {
            // Create default config if it doesn't exist
            let default_config = Self::default();
            if config_path.is_none() {
                default_config.save(config_file.as_ref()).await?;
            }
            return Ok(default_config);
        }

        let content = fs::read_to_string(config_file.as_ref())
            .await
            .with_context(|| format!("Could not read config file: {}", config_file.display()))?;

        Self::from_ini(&content)
    }

    /// Save configuration to file
    pub async fn save(&self, path: &Path) -> Result<()> {
        let content = to_ini(
            &self.default_type,
            self.create_folder,
            self.enable_hooks,
            &self.templates_dir,
            &self.output_dir,
            &self.architectures_dir,
            &self.default_architecture,
        );

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await.with_context(|| {
                format!("Failed to create parent directory: {}", parent.display())
            })?;
        }

        fs::write(path, content)
            .await
            .with_context(|| format!("Could not save config file: {}", path.display()))?;

        Ok(())
    }

    /// Parse configuration from INI format
    fn from_ini(content: &str) -> Result<Self> {
        let mut config = Self::default();

        let pairs = parse_ini(content);
        for (key, value) in pairs {
            match key.as_str() {
                "default_type" => config.default_type = value,
                "create_folder" => config.create_folder = value.parse().unwrap_or(true),
                "enable_hooks" => config.enable_hooks = value.parse().unwrap_or(true),
                "templates_dir" => config.templates_dir = expand_path(&value)?,
                "output_dir" => config.output_dir = PathBuf::from(value),
                "architectures_dir" => config.architectures_dir = expand_path(&value)?,
                "default_architecture" => config.default_architecture = value,
                _ => {} // Ignore unknown keys
            }
        }

        Ok(config)
    }
}
