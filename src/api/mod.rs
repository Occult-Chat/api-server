use anyhow::anyhow;
use anyhow::Result;
use log::error;
use log::info;
use rocket::get;
use rocket::routes;

use crate::workspace::ServerConfig;
#[get("/")]
fn identity() -> &'static str {
    "Hello, world!"
}

#[get("/get/<package>/<version>")]
fn get_package(package: &str, version: &str) -> String {
    info!("requested package name: '{package}' requested package version: '{version}'");
    let response = format!("{package}V{version}");
    response
}

pub async fn start_listener(config: &ServerConfig) -> Result<()> {
    let log_level = &config.log_level.as_str().to_lowercase();
    println!("Starting occult server. Current log level: {log_level}");
    log::set_max_level(config.log_level);
    let _server = rocket::build()
        .configure(rocket::config::Config {
            port: config.port,

            ..Default::default()
        })
        .mount("/", routes![identity, get_package])
        .launch()
        .await
        .map_err(|e| {
            error!("An error occured. {e}");
            anyhow!(format!("Failed to start rocket server: {e:#}"))
        });
    Ok(())
}
