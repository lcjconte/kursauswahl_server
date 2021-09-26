use std::env;
use dotenv::dotenv;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::{MakeTlsConnector};
use tokio_postgres::{Client};
use std::error::Error;
use std::fmt;

use crate::structures::User;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

//static conn_slots: Vec<Option<(Client, Connection<Socket>)>> = vec![None;19]; //TODO: DB only allows 20 connections!

#[derive(Debug)]
struct DBError{

}
impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in database")
    }
}
impl Error for DBError {}

async fn get_url() -> Result<String>{
    if let Err(e) = dotenv() {eprintln!("Error reading .env");}
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

pub async fn get_user(uname: &str) -> Result<Option<User>> {  //DOUBLE!!
    let client= connect().await?;
    let rows = client
        .query("SELECT * FROM users WHERE username = $1", &[&uname])
        .await?;
    if rows.len() == 0{
        Ok(None)
    }
    else {
        Ok(Some(User {
            id: rows[0].get("id"),
            username: rows[0].get("username"), 
            pwdhash: rows[0].get("pwdhash"),
            is_admin: rows[0].get("isAdmin")}))
    }
}

pub async fn user_by_id(id: i32) -> Result<Option<User>> {
    let client= connect().await?;
    let rows = client
        .query("SELECT * FROM users WHERE id = $1", &[&id])
        .await?;
    if rows.len() == 0{
        Ok(None)
    }
    else {
        Ok(Some(User {
            id: rows[0].get("id"),
            username: rows[0].get("username"), 
            pwdhash: rows[0].get("pwdhash"),
            is_admin: rows[0].get("isAdmin")}))
    }
}

pub async fn add_user(user: User) -> Result<()>{
    let client = connect().await?;
    let changed = client.execute("INSERT INTO users VALUES (DEFAULT, $1, $2, $3)", &[&user.username, &user.pwdhash, &user.is_admin]).await?;
    if changed==1 { Ok(()) } else {Err(Box::new(DBError{}))}
}