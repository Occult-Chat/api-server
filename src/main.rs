#![feature(thread_id_value)]
use anyhow::Result;
use cli::run_cli;

pub mod api;
pub mod cli;
pub mod workspace;
pub mod logger;
pub mod user;
pub mod db;

#[rocket::main]
async fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    {
        std::env::set_var("RUST_LOG", "debug");
    }
    logger::init().expect("Failed to initalize logger");
    run_cli().await?;

    Ok(())
}
