use std::env;
use crate::config::load_config;

mod config;

fn main() {
    let config_path = env::current_dir().unwrap();
    let config_path = config_path.join("config.toml");
    if !config_path.exists() {
        panic!("{} is not exist",config_path.as_os_str().to_str().unwrap());
    }
    let new_config = load_config(&config_path).unwrap();
    println!("{:#?}", new_config);
}
