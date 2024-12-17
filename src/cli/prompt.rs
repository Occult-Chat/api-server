use std::{path::PathBuf, str::FromStr};

use crate::workspace::{self, get_server_dir, Port, ServerConfig};
use anyhow::{Context, Result};
use inquire::{Confirm, Password, Select, Text};

use log::debug;

pub fn create_config_interactive() -> Result<ServerConfig> {
    let http_port: Port = Text::new("Enter the port number you would like the server to run on:")
        .with_default("8000")
        .prompt()?
        .parse()?;

    let db_url = Text::new("At what IP address is your database hosted?")
        .with_initial_value("localhost")
        .prompt()
        .unwrap();

    let db_port: Port = Text::new("Enter the port number you would like connect to for the DB:")
        .with_default("3306")
        .prompt()?
        .parse()?;
    let db_user = Text::new("What would you like the user credentials for DB to be?")
        .with_default("root")
        .prompt()?
        .parse()?;
    let db_pass = Password::new("What would you like the DB password to be?")
        .with_display_toggle_enabled()
        .prompt()?
        .parse()?;
    let db_name = Text::new("What would you like the db name to be?")
    .with_default("occult_db")
    .prompt()?
    .parse()?;
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

    let env_override = Confirm::new(
        "Would you like to allow enviornment variables to override configuration options?",
    )
    .with_default(true)
    .with_help_message("WARNING: DISABLING THIS CAN BREAK AUTOMATION (such as a docker compose)")
    .prompt()?;

    
    let config = ServerConfig {
        http_port,
        use_http,
        log_level,
        log_path,
        db_url,
        env_override,
        db_port,
        db_user,
        db_pass,
        db_name,


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
