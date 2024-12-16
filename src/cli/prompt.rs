use std::{path::PathBuf, str::FromStr};

use crate::workspace::{self, get_server_dir, ServerConfig};
use anyhow::{Context, Result};
use inquire::{Confirm, Select, Text};

use log::debug;
use url::Url;

pub fn create_config_interactive() -> Result<ServerConfig> {
    let port: u16 = Text::new("Enter the port number you would like the server to run on:")
        .with_default("8000")
        .prompt()?
        .parse()?;

    let input_url = Text::new("What will the  for your repository be?")
        .with_initial_value("https://")
        .prompt()
        .unwrap();
    let db_url = Url::from_str(&input_url)?;
    let use_http = Confirm::new("Would you like to enable HTTP?")
        .with_default(false)
        .prompt()?;
    let log_level = Select::new(
        "What log level would you like the server to use",
        vec![
            log::LevelFilter::Trace,
            log::LevelFilter::Error,
            log::LevelFilter::Warn,
            log::LevelFilter::Debug,
            log::LevelFilter::Info,
        ],
    )
    .prompt()?;
    let log_path: Option<PathBuf> = Text::new("Where do you want to store your logs?")
        .prompt_skippable()
        .map(|s| {
            if s.is_some() {
                let path = PathBuf::from_str(&s.unwrap()).ok();
                return path;
            } else {
                None
            }
        })?;

    let env_override = Confirm::new("Would you like to allow enviornment variables to override configuration options?")
    .with_default(true)
    .with_help_message("WARNING: DISABLING THIS WILL BREAK AUTOMATION (such as a docker)")
    .prompt()?;
    let config = ServerConfig {
        port,

        use_http,
        log_level,
        log_path,
        db_url,
        env_override,

    };
    let config_path = get_server_dir().context("Failed to obtain config path")?;

    let config_content =
        serde_yml::to_string(&config).context("Failed to serialize config content")?;
    debug!("config: {config_content:#}");
    workspace::write_to_path(&config_path, config_content, "config.server.yml")
        .context("Failed to write new configuration to server.config.yml")?;
    Ok(config)
}

pub fn create_config() -> ServerConfig {
    ServerConfig::default()
}

