mod tableconfig;
mod queries;
use crate::cli::*;

use crate::workspace::ServerConfig;
use anyhow::Result;
use log::debug;
use sqlx::{
    database, error,
    mysql::{self, MySqlPoolOptions},
    pool, Acquire, Pool,
};

pub struct Db {}

pub struct MySqlConnect {
    pub pool: Pool<sqlx::MySql>,
}

impl MySqlConnect {
    pub async fn connect(config: &ServerConfig) -> Result<Self, sqlx::Error> {
        let connection_string = format!(
            "mysql://{}:{}@{}:{}/{}",
            config.db_user, config.db_pass, config.db_url, config.db_port, config.db_name
        );

        println!("Connection string: {:#?}", connection_string);

        let pool = MySqlPoolOptions::new().connect(&connection_string).await?;

        Ok(Self { pool })
    }
}

async fn db_setup(config: &ServerConfig) -> MySqlConnect {
    debug!("attempt to connect to the db!");
    let db_connect = MySqlConnect::connect(config)
        .await
        .expect("ERROR: MySql CONNECTION FAILURE");

    let mut connection = db_connect
        .pool
        .acquire()
        .await
        .expect("Failed to acquire connection from the DB pool");
    debug!("Connected to db!");
    let mut transaction = connection
        .begin()
        .await
        .expect("Failed to begin transaction");

    // Attempt to set up the table structure for the app
    match tableconfig::init_tables(&mut transaction).await {
        Ok(_) => {
            transaction.commit().await.expect(
                "We were unable to commit the data to disk and are crashing, generally this indicates a connection issue, no data was not written to the disk so your data should be OK"
            );
        }
        Err(_) => {
            transaction.rollback().await.expect(
                "We were unable to drop the transaction, and are crashing. Data was not written to disk so your data should be OK"
            )
        }
    }

    // These are testing creds, dont even try it
    // Pass the pool reference directly, not a connection
    queries::register_user(&db_connect.pool, "Caz", "admin@occult.chat", "oeisntjvlketnvdfhdlgnjflkjdhl")
        .await
        .expect("Failed to register user"); // Added error handling

    db_connect
}

pub async fn start_db(config: &ServerConfig) {
    log::error!("Starting DB on port {:#?}", config.db_port);
    db_setup(&config).await;
}