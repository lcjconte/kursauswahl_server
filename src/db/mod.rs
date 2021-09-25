use std::env;
use dotenv::dotenv;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::{MakeTlsConnector, TlsStream};
use tokio_postgres::{Client, Socket, error::ErrorPosition, Connection};

use crate::structures::User;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

//static conn_slots: Vec<Option<(Client, Connection<Socket>)>> = vec![None;19]; //TODO: DB only allows 20 connections!

async fn get_url() -> Result<String>{
    if let Err(e) = dotenv() {eprintln!("Error reading .env");return Err(e.into())}
    env::var("DATABASE_URL").map_err(|op| {eprintln!("Error fetching database url");op.into()})
}

async fn connect() -> Result<Client> {
    let mut builder = SslConnector::builder(SslMethod::tls())?;
    builder.set_verify(SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(builder.build());
    let conurl = get_url().await?;
    let (client, connection) = tokio_postgres::connect(&conurl, connector).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}

async fn getUser(uname: &str) -> Result<Option<User>> {
    let client= connect().await?;
    let rows = client
        .query("SELECT * FROM users WHERE username = $1", &[&uname])
        .await?;
    if rows.len() == 0{
        Ok(None)
    }
    else {
        Ok(Some(User {
            username: rows[0].get("username"), 
            pwdhash: rows[0].get("pwdhash"),
            isAdmin: rows[0].get("isAdmin")}))
    }
}