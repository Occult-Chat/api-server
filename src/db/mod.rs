use crate::cli::*;

use tokio::{
    io::{AsyncWrite,AsyncRead},
 //    net::
};
use sqlx::{ database, error, mysql::{self, MySqlPoolOptions}, pool, Pool };
use anyhow::Result;
use crate::workspace::ServerConfig;
pub struct Db {
}

pub struct MySqlConnect {
    pub pool: Pool<sqlx::MySql>,
}

impl MySqlConnect {
    pub async fn connect(config: &ServerConfig) -> Result<Self, sqlx::Error> {
        let connection_string = format!(
            "mysql://{}:{}@{}:{}/{}",
            config.db_user,
            config.db_pass,
            config.db_url,
            config.db_port,
            config.db_name
        );

        println!("Connection string: {:#?}", connection_string);
        
        let pool = MySqlPoolOptions::new()
            .connect(&connection_string).await?;
            
        Ok(Self { pool })
    }
}


async fn db_setup(config: &ServerConfig) -> MySqlConnect {
    println!("Connected to db!");
    let db_connect = MySqlConnect::connect(config)
        .await
        .expect("ERROR: MySql CONNECTION FAILURE");

        println!("Connected to db!");

        
//    dbConnect.
}

/* pub async fn print_db_connect(&self) -> Result<sqlx::pool::PoolConnection<MySql>, sqlx::Error> {
    self.pool.acquire().await?;
} */


pub async fn start_db(config: &ServerConfig) {
    log::error!("Starting DB on port {:#?}",config.db_port);
    db_setup(&config).await;
}