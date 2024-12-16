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
        
        let pool =  MySqlPoolOptions::new()
            .connect(config.db_url.as_str()).await?;
        Ok(Self { pool })
    }
}


async fn db_setup(config: &ServerConfig) -> MySqlConnect {
    MySqlConnect::connect(config)
        .await
        .expect("ERROR: MySql CONNECTION FAILURE")
//    dbConnect.
}

/* pub async fn print_db_connect(&self) -> Result<sqlx::pool::PoolConnection<MySql>, sqlx::Error> {
    self.pool.acquire().await?;
} */


pub async fn start_db(config: &ServerConfig) {
    log::error!("Starting DB on port {}",config.port);
    db_setup(&config).await;
}