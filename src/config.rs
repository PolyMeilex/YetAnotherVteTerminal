use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_colors")]
    pub colors: Vec<(u8, u8, u8)>,
    #[serde(default = "default_alpha")]
    pub alpha: f64,
}

impl Config {}

fn default_colors() -> Vec<(u8, u8, u8)> {
    vec![
        (40, 44, 52),
        (224, 108, 117),
        (152, 195, 121),
        (229, 192, 123),
        (97, 175, 239),
        (198, 120, 221),
        (86, 182, 194),
        (171, 178, 191),
    ]
}

fn default_alpha() -> f64 {
    0.7
}

pub fn get_config() -> Config {
    let config_dir = config_dir();

    let default_config = include_str!("./config.ron");

    if let Some(config_dir) = config_dir {
        let term_config_dir = config_dir.join("YetAnotherVteTerminal");
        let _ = std::fs::create_dir_all(&term_config_dir);

        if let Ok(file) = std::fs::read_to_string(&term_config_dir.join("config.ron")) {
            ron::from_str(&file).unwrap()
        } else {
            ron::from_str(default_config).unwrap()
        }
    } else {
        ron::from_str(default_config).unwrap()
    }
}

use std::path::PathBuf;
fn config_dir() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").and_then(|h| if h.is_empty() { None } else { Some(h) });
    if let Some(home) = home {
        Some(PathBuf::from(home).join(".config"))
    } else {
        None
    }
}
