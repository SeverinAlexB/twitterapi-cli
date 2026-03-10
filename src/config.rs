use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub api_key: Option<String>,
    pub no_cache: bool,
    pub json_output: bool,
    pub cache_dir: PathBuf,
}

#[derive(Debug, Deserialize, Default)]
struct ConfigFile {
    api_key: Option<String>,
}

impl AppConfig {
    pub fn load(api_key: Option<String>, no_cache: bool, json_output: bool) -> Self {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from(".cache"))
            .join("twitterapi-cli");

        let config_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".config")
            .join("twitterapi-cli");

        let file_config = load_config_file(&config_dir);

        // Priority: CLI flags > env vars > config file
        let api_key_env = std::env::var("TWITTERAPI_API_KEY").ok();
        let api_key = api_key.or(api_key_env).or(file_config.api_key);

        AppConfig {
            api_key,
            no_cache,
            json_output,
            cache_dir,
        }
    }
}

fn load_config_file(config_dir: &Path) -> ConfigFile {
    let config_path = config_dir.join("config.toml");
    if config_path.exists() {
        match std::fs::read_to_string(&config_path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => ConfigFile::default(),
        }
    } else {
        ConfigFile::default()
    }
}
