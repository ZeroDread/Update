use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app_name: String,
    pub version: String,
    pub author: String,
}

impl Config {
    pub fn load() -> Config {
        let config_path = Path::new("config.json");
        if config_path.exists() {
            let data = fs::read_to_string(config_path).expect("Unable to read configuration file.");
            serde_json::from_str(&data).expect("Invalid configuration format.")
        } else {
            Config::default()
        }
    }

    pub fn default() -> Config {
        Config {
            app_name: "System Update Manager".into(),
            version: "0.1.0".into(),
            author: "z3r0dr34d".into(),
        }
    }

    pub fn save(&self) {
        let data = serde_json::to_string_pretty(self).expect("Failed to serialize configuration.");
        fs::write("config.json", data).expect("Unable to write configuration file.");
    }
}
