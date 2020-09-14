use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub colors: Vec<(u8, u8, u8)>,
}

impl Config {}

fn default_colors() -> Vec<(u8, u8, u8)> {
    vec![
        (255, 0, 0),
        (0, 255, 0),
        (0, 0, 255),
        (255, 0, 0),
        (0, 255, 0),
        (0, 0, 255),
        (255, 0, 0),
        (0, 255, 0),
    ]
}

pub fn get_config() -> Config {
    ron::from_str(include_str!("./config.ron")).unwrap()
}
