use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;

use log::debug;
use log::error;
use serde::Deserialize;
use serde::Serialize;
use url::Url;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid port number. expected value in range 1..65535 got {received}")]
    InvalidPort { received: u32 },
    #[error("Failed to parse value. Expected {expected}. Got: {received}")]
    FailedToParse { expected: String, received: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Port(u32);

impl Port {
    pub fn new(port: u32) -> Result<Self, ConfigError> {
        if port >= 1 && port <= 65535 {
            Ok(Self(port))
        } else {
            Err(ConfigError::InvalidPort { received: port })
        }
    }
    pub fn get(&self) -> u32 {
        self.0
    }
}

impl FromStr for Port {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(port) => {
                if port >= 1 && port <= 65535 {
                    Ok(Port(port))
                } else {
                    Err(ConfigError::InvalidPort { received: port })
                }
            }
            Err(_) => Err(ConfigError::FailedToParse {
                expected: String::from("Valid u16"),
                received: String::from("port"),
            }),
        }
    }
}

impl From<Port> for u32 {
    fn from(value: Port) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    // Base url the repository is hosted at
    pub db_url: Url,
    // Define if the server has a cert
    pub use_http: bool,
    pub log_level: log::LevelFilter,
    pub log_path: Option<PathBuf>,
    pub env_override: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {

        Self {

            port: 8000,
            db_url: Url::from_str("https://0.0.0.0:8000").unwrap(),
            use_http: true,
            log_level: log::LevelFilter::Info,
            log_path: Some(PathBuf::from_str("./").unwrap()),
            env_override: true
        }
    }
}

pub fn get_config() -> Result<Option<ServerConfig>> {
    let mut config_path = get_working_dir()?;
    if !config_path.exists() {
        debug!("the occult config path did not yet exist, this is likely the first run");
        return Ok(None);
    }
    config_path.push("server/config.server.yml");
    if !config_path.exists() {
        debug!("the server config did not exist: {config_path:#?}");
        return Ok(None);
    }
    let config =
        fs::read_to_string(config_path).context("Failed to read configuration file to string")?;
    match serde_yml::from_str(&config) {
        Ok(config) => {
            debug!("server config has been serialized: Config: {config:#?}");
            return Ok(Some(config));
        }
        Err(e) => {
            error!("Could not serialize configuration: {e}");
            return Err(e.into());
        }
    };
}

pub fn is_initalized() -> bool {
    let mut config_path = get_working_dir().unwrap();
    config_path.push("server/config.server.yml");

    match get_config() {
        Ok(config) => {
            if config.is_some() {
                true
            } else {
                false
            }
        }
        Err(e) => {
            error!("An error has occured, assuming we are not initalized: {e}");
            false
        }
    }
}

pub fn write_to_path(path: &PathBuf, content: String, file_name: &str) -> Result<()> {
    if !path.exists() {
        debug!("Path did not yet exist: {path:#?}");
        fs::create_dir_all(path)?;
    }
    let mut new_path = path.clone();
    new_path.push(file_name);
    debug!("Full file path: {new_path:#?}");
    let mut file = File::create(&new_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
pub fn get_working_dir() -> Result<PathBuf> {
    let mut dir = dirs_next::data_dir()
        .ok_or(anyhow!("Expected working directory, found: None"))
        .context("Could not fetch working dir")?;
    dir.push("occult");
    Ok(dir)
}

pub fn get_data_dir() -> Result<PathBuf> {
    let mut dir = get_working_dir().context("Failed to obtain working dir")?;
    dir.push("data");
    Ok(dir)
}
pub fn get_server_dir() -> Result<PathBuf> {
    let mut dir = get_working_dir().context("Failed to obtain server dir")?;
    dir.push("server");
    Ok(dir)
}
