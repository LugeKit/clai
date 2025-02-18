use anyhow::anyhow;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Config {
    pub prompt: String,
    pub timeout: u64,
    pub access_point: String,
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
        let default_prompt = String::from("you are an excellent programmer, now briefly answer the questions, don't explain your answer in details when it is not required");
        let default_timeout = 1800;

        match config {
            Some(config) => Config {
                prompt: if !config.prompt.is_empty() {
                    config.prompt
                } else {
                    default_prompt
                },
                timeout: if config.timeout > 0 {
                    config.timeout
                } else {
                    default_timeout
                },
                access_point: config.access_point,
            },
            None => Config {
                prompt: default_prompt,
                timeout: default_timeout,
                access_point: String::new(),
            },
        }
    }
}
