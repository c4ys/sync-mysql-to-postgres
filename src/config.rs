use std::error::Error;
use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub from: String,
    pub to: String,
    pub tables: Vec<String>,
    pub limit: i32,
    pub sleep: i32,
}

pub fn load_config(path: &PathBuf) -> Result<Config, Box<dyn Error>> {
    let serialized = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&*serialized)?;
    Ok(config)
}