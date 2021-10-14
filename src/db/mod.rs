use std::env;
use dotenv::dotenv;
use anyhow::{Context, Result};
use std::error::Error;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::{MakeTlsConnector};
use tokio_postgres::{Client};
use tokio::sync::{RwLock};
use lazy_static::lazy_static;

mod user_system;
pub use user_system::*;
mod selection_system;
pub use selection_system::*;
mod course_system;

#[derive(Debug)]
pub enum DBError {
    Conflict,
}
impl std::fmt::Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for DBError {}
    
//static conn_slots: Vec<Option<(Client, Connection<Socket>)>> = vec![None;19]; //TODO: DB only allows 20 connections!
lazy_static! {
    static ref ACTIVE_CONNECTIONS: RwLock<u32> = RwLock::new(0);
}

async fn get_url() -> Result<String>{
    if let Err(_) = dotenv() {eprintln!("Error reading .env");}
    env::var("DATABASE_URL").context("Couldn't retrieve env var".to_string())
}

async fn connect() -> Result<Client> {
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(builder.build());
    let conurl = get_url().await?;
    let (client, connection) = tokio_postgres::connect(&conurl, connector).await.context("Connection Error")?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("DB connection error: {}", e);
        }
    });
    Ok(client)
}

