use std::fs;
use std::path::PathBuf;

use super::settings::AppSettings;

/// Configuration loader for application settings
pub struct ConfigLoader {
    config_path: PathBuf,
}

impl ConfigLoader {
    /// Create a new config loader with default path
    pub(crate) fn new() -> Self {
        let config_path = Self::default_config_path();
        Self { config_path }
    }

    /// Create a new config loader with custom path
    pub(crate) const fn with_path(path: PathBuf) -> Self {
        Self { config_path: path }
    }

    /// Get default configuration path
    fn default_config_path() -> PathBuf {
        // Use XDG config directory or fallback to .config
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from(".config"));
        
        config_dir.join("agent-tui").join("config.toml")
    }

    /// Load configuration from file
    pub(crate) fn load(&self) -> Result<AppSettings, ConfigError> {
        if !self.config_path.exists() {
            // Return default settings if config doesn't exist
            return Ok(AppSettings::default());
        }

        let content = fs::read_to_string(&self.config_path)
            .map_err(|e| ConfigError::ReadError(e.to_string()))?;

        let settings: AppSettings = toml::from_str(&content)
            .map_err(|e| ConfigError::ParseError(e.to_string()))?;

        Ok(settings)
    }

    /// Save configuration to file
    pub(crate) fn save(&self, settings: &AppSettings) -> Result<(), ConfigError> {
        // Ensure parent directory exists
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| ConfigError::WriteError(e.to_string()))?;
        }

        let content = toml::to_string_pretty(settings)
            .map_err(|e| ConfigError::SerializeError(e.to_string()))?;

        fs::write(&self.config_path, content)
            .map_err(|e| ConfigError::WriteError(e.to_string()))?;

        Ok(())
    }
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    ReadError(String),
    
    #[error("Failed to parse config: {0}")]
    ParseError(String),
    
    #[error("Failed to serialize config: {0}")]
    SerializeError(String),
    
    #[error("Failed to write config file: {0}")]
    WriteError(String),
}
