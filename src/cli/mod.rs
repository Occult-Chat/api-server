use crate::{
    api::start_listener,
    db::start_db,
    workspace::{get_config, is_initalized, ServerConfig},
};
use anyhow::{Context, Ok, Result};
use clap::{Parser, Subcommand};
use inquire::Confirm;
use log::{debug, info};
use prompt::create_config_interactive;
pub mod prompt;

#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
struct Cli {
    #[arg(short, long)]
    // Disables interactive prompts
    unattended: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short)]
        // automatically accepts defaults
        y: bool,
    },
    Run {
        #[arg(short, long)]
        /// Runs the server as a background process
        daemon: bool,
    },
}

pub async fn run_cli() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { y }) => match (is_initalized(), y) {
            (true, false) => {
                let confirm = Confirm::new(
                    "A configuration file already exists, are you sure you want to overwrite it?",
                )
                .with_default(false)
                .prompt()?;

                match confirm {
                    false => std::process::exit(0),
                    true => {
                        create_config_interactive()?;
                        println!("Use occult-server run to start your newly configured server!");
                        Ok(())
                    }
                }
            }
            (false, false) => {
                if !get_config().is_ok() {
                    let confirm = Confirm::new(
                            "You are about to overwrite an invalid configuration. Are you sure this is what you want?"
                        )
                            .with_default(false)
                            .prompt()?;
                    if !confirm {
                        std::process::exit(0);
                    }
                }
                create_config_interactive()?;
                Ok(())
            }
            _ => {
                unreachable!();
            }
        },
        Some(Commands::Run { daemon }) => {
            if daemon {
                debug!("Server will be started as a background process");
            }
            info!("Running server");
            let config = get_config().unwrap_or_else(|e| {
                let should_reconfigre =
                    Confirm::new("Your configuration is invalid. would you like to reconfigure?")
                        .with_default(true)
                        .with_help_message(format!("{e}").as_str())
                        .prompt()
                        .expect("Failed to obtain user confirmation");
                if should_reconfigre {
                    create_config_interactive().expect("")
                } else {
                    eprintln!("Please edit your server.config.yml and correct the above errors");
                    std::process::exit(0)
                }
            });
            debug!("server_config = {config:#?}");
            let listener = start_listener(&config);
            let db = start_db(&config);
            tokio::join!(listener, db).0.unwrap_or_else(|e| loop {
                
            });
            Ok(())
        }
        None => {
            println!("No command provided. Use 'occult-server --help' for usage information.");
            std::process::exit(0);
        }
    }
}
