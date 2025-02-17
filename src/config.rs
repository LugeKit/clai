use anyhow::anyhow;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
pub struct Config {
    pub prompt: String,
    pub timeout: u64,
}

impl Config {
    pub fn new() -> Config {
        let config_from_file = Self::read_from_file();
        match config_from_file {
            Ok(config) => {
                config
            }
            Err(_) => {
                Config {
                    prompt: "you are an excellent programmer, now breifly answer the questions, don't explain your answer in details when it is not required".to_string(),
                    timeout: 60,
                }
            }
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
}
