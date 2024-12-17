use std::fmt::write;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

use log::debug;
use log::error;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid port number. expected value in range 1..65535 got {received}")]
    InvalidPort { received: u32 },
    #[error("Failed to parse value. Expected {expected}. Got: {received}")]
    FailedToParse { expected: String, received: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl std::fmt::Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",self.0)
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
    // Define if the server has a cert
    pub http_port: Port,
    pub use_http: bool,
    pub log_level: log::LevelFilter,
    pub log_path: Option<PathBuf>,
    pub env_override: bool,
    
    // Base url the DB is hosted at
    pub db_url: String,
    pub db_port: Port,
    
    // Credentials for the database
    pub db_user: String,
    pub db_pass: String,
    pub db_name: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            http_port: Port(8000),
            db_url: "localhost".to_string(),
            use_http: true,
            log_level: log::LevelFilter::Info,
            log_path: Some(PathBuf::from_str("./").unwrap()),
            env_override: true,
            db_port: Port(3306),
            db_user: "occult".to_string(),
            db_pass: "occult".to_string(),
            db_name: "occult_db".to_string()
        }
    }
}

pub fn get_config() -> Result<ServerConfig> {
    let mut config_path = get_working_dir()?;
    if !config_path.exists() {
        error!("the occult config path did not yet exist, this is likely the first run");
        return Err(anyhow!(
            "the occult config path did not yet exist, this is likely the first run"
        ));
    }
    config_path.push("server/config.server.yml");
    if !config_path.exists() {
        error!("the server config did not exist: {config_path:#?}");
        return Err(anyhow!("the server config did not exist: {config_path:#?}"));
    }
    let config =
        fs::read_to_string(config_path).context("Failed to read configuration file to string")?;
    match serde_yml::from_str(&config) {
        Ok(config) => {
            debug!("server config has been serialized: Config: {config:#?}");
            return Ok(config);
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
        Ok(_) => true,
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
