use actix_web::cookie::Key;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(with = "hex")]
    pub key: Vec<u8>,
}

fn read_config(path: &String) -> Config {
    let config_str = fs::read_to_string(path).unwrap();
    toml::from_str(&config_str).unwrap()
}

fn save_config(path: &String, config: &Config) {
    let config_str = toml::to_string_pretty(config).unwrap();
    fs::write(path, config_str).unwrap();
}

pub fn load_config(workdir_path: &String) -> Config {
    let config_path = workdir_path.to_string() + "/config.toml";
    log::info!("Config path: {}", config_path);
    if fs::exists(&config_path).unwrap() {
        read_config(&config_path)
    } else {
        let config = Config {
            key: Key::generate().master().into(),
        };
        save_config(&config_path, &config);
        config
    }
}
