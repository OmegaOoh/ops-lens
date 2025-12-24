use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeybindConfig {
    pub quit: String,
    pub edit: String,
    pub scroll_up: String,
    pub scroll_down: String,
}

impl Default for KeybindConfig {
    fn default() -> Self {
        Self {
            quit: "q".to_string(),
            edit: "e".to_string(),
            scroll_up: "k".to_string(),
            scroll_down: "j".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogLevelConfig {
    pub enabled: bool,
    pub indicator_position: String, // "top" or "bottom"
}

impl Default for LogLevelConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            indicator_position: "top".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub keybinds: KeybindConfig,
    pub log_level: LogLevelConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            keybinds: KeybindConfig::default(),
            log_level: LogLevelConfig::default(),
        }
    }
}

impl Config {
    pub fn load_or_create(config_path: &str) -> Self {
        // Try to load existing config
        if Path::new(config_path).exists() {
            match fs::read_to_string(config_path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => {
                        println!("✓ Configuration loaded from: {}", config_path);
                        return config;
                    }
                    Err(e) => {
                        eprintln!("⚠ Failed to parse config file: {}. Using defaults.", e);
                    }
                },
                Err(e) => {
                    eprintln!("⚠ Failed to read config file: {}. Using defaults.", e);
                }
            }
        }

        // Create default config
        let default_config = Self::default();

        // Try to create the config file with defaults
        match fs::write(
            config_path,
            toml::to_string_pretty(&default_config).unwrap(),
        ) {
            Ok(_) => {
                println!("✓ Default configuration created at: {}", config_path);
            }
            Err(e) => {
                eprintln!(
                    "⚠ Failed to create config file: {}. Using in-memory defaults.",
                    e
                );
            }
        }

        default_config
    }

    #[allow(dead_code)]
    pub fn save(&self, config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = toml::to_string_pretty(self)?;
        fs::write(config_path, config_str)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.keybinds.quit, "q");
        assert_eq!(config.keybinds.edit, "e");
        assert_eq!(config.keybinds.scroll_up, "k");
        assert_eq!(config.keybinds.scroll_down, "j");
        assert!(config.log_level.enabled);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(config.keybinds.quit, parsed.keybinds.quit);
    }
}
