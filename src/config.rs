use anyhow::anyhow;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Config {
    pub prompt: String,
    pub timeout: u64,
    pub default_model: String,
    pub base_url: String,
    pub models: HashMap<String, Model>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Model {
    pub access_point: String,
    pub base_url: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        let config_from_file = Self::read_from_file();
        match config_from_file {
            Ok(config) => Self::fill_config(Some(config)),
            Err(_) => Self::fill_config(None),
        }
    }

    fn read_from_file() -> anyhow::Result<Config> {
        let home_path = dirs::home_dir().ok_or(anyhow!("failed to load home dir"))?;
        let mut config_file = File::open(home_path.join(".config/clai/config.json"))?;
        let mut file_content = String::new();
        config_file.read_to_string(&mut file_content)?;

        let config: Config = serde_json::from_str(&file_content)?;
        Ok(config)
    }

    fn fill_config(config: Option<Config>) -> Config {
        let default_timeout = 1800;
        match config {
            Some(config) => Config {
                prompt: config.prompt,
                timeout: if config.timeout > 0 {
                    config.timeout
                } else {
                    default_timeout
                },
                default_model: config.default_model,
                base_url: config.base_url,
                models: config.models,
            },
            None => Config {
                prompt: String::new(),
                timeout: default_timeout,
                default_model: String::new(),
                base_url: String::new(),
                models: HashMap::new(),
            },
        }
    }
}
