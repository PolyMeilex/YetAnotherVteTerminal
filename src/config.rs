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
    ron::from_str(include_str!("./config.ron")).unwrap()
}
