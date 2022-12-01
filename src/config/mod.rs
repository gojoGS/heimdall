use crate::log::LogLevel;
use config::{Config, ConfigError};
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub min_log_level: LogLevel,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            min_log_level: LogLevel::Info,
        }
    }
}

fn get_config_from_path(path: &Path) -> Result<Config, ConfigError> {
    Config::builder()
        .add_source(config::File::with_name(path.as_os_str().to_str().unwrap()))
        .add_source(config::Environment::with_prefix("HEIMDALL"))
        .build()
}

fn parse_config(config: Config) -> AppConfig {
    let settings_result = config.try_deserialize::<HashMap<String, String>>();

    if settings_result.is_err() {
        return AppConfig::default();
    }

    let settings = settings_result.unwrap();

    let min_log_level = match settings.get("min_log_level") {
        Some(log_level) => LogLevel::from_str(log_level),
        None => LogLevel::Info,
    };

    AppConfig { min_log_level }
}

pub fn load_config(path: &PathBuf) -> Result<AppConfig, io::Error> {
    match get_config_from_path(path) {
        Ok(config) => Ok(parse_config(config)),
        Err(_) => Err(io::Error::new(ErrorKind::Other, "couldn't load config")),
    }
}

pub fn get_config_path() -> &'static PathBuf {
    lazy_static! {
        static ref home_dir: PathBuf = dirs::home_dir().unwrap();
        static ref path: PathBuf =
            home_dir.join(format!(".heimdall{}config.json", std::path::MAIN_SEPARATOR));
    }
    println!("{}", path.as_os_str().to_str().unwrap());

    &*path
}

pub fn create_config_if_not_exists() -> Result<(), io::Error> {
    let path = get_config_path();

    println!("{}", path.as_os_str().to_str().unwrap());

    if path.exists() && path.is_file() {
        println!("path exists");
        return Ok(());
    }

    fs::create_dir_all(path.parent().unwrap())?;

    let file_contents = serde_json::to_string(&AppConfig::default())?;

    println!("{}", path.as_os_str().to_str().unwrap());

    {
        let mut file = File::create(path)?;
        file.write_all(file_contents.as_bytes())?;
    }

    Ok(())
}
