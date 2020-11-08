use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_colors")]
    pub colors: Vec<(u8, u8, u8)>,
}

impl Config {}

fn default_colors() -> Vec<(u8, u8, u8)> {
    vec![
        (0, 3, 47),
        (93, 108, 142),
        (151, 121, 143),
        (68, 163, 184),
        (102, 213, 211),
        (162, 160, 172),
        (222, 161, 153),
        (255, 255, 255),
    ]
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
