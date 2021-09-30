use std::env;
use dotenv::dotenv;
use anyhow::{Context, Result};
use std::error::Error;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::{MakeTlsConnector};
use tokio_postgres::{Client};
use tokio::sync::{RwLock};
use lazy_static::lazy_static;

use crate::slogic::{User, IUser};

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

pub async fn get_user(uname: &str) -> Result<Option<User>> {  //DOUBLE!!
    let client = connect().await?;
    let rows = client
        .query("SELECT * FROM users WHERE username = $1", &[&uname])
        .await.context("Query Error")?;
    if rows.len() == 0{
        Ok(None)
    }
    else {
        Ok(Some(User::new(rows[0].get("id"),rows[0].get("username"),rows[0].get("pwdhash"),rows[0].get("isAdmin"))))
    }
}

pub async fn user_by_id(id: i32) -> Result<Option<User>> {
    let client= connect().await?;
    let rows = client
        .query("SELECT * FROM users WHERE id = $1", &[&id])
        .await.context("Query Error")?;
    if rows.len() == 0{
        Ok(None)
    }
    else {
        Ok(Some(User::new(rows[0].get("id"),rows[0].get("username"),rows[0].get("pwdhash"),rows[0].get("isAdmin"))))
    }
}

pub async fn add_user(user: &User) -> Result<()>{
    let client = connect().await?;
    let changed = client.execute("INSERT INTO users VALUES (DEFAULT, $1, $2, $3)", &[user.username(), user.pwdhash(), &user.is_admin()]).await
        .context("Execute error")?;
    if changed==1 { Ok(()) } else {Err(DBError::Conflict.into())}
}

pub enum UserUpdate {
    Username(String),
    Pwdhash(String),
    IsAdmin(bool),
}

pub async fn update_user(id: i32, update: UserUpdate) -> Result<Option<u64>> {
    let client = connect().await?;
    if user_by_id(id).await?.is_none() {
        return Ok(None);
    }
    match update {
        UserUpdate::Username(name) => {
            if let Some(_conflict) = get_user(&name).await? {
                return Err(DBError::Conflict.into());
            }
            client.execute("UPDATE users SET username=$1 where id=$2", &[&name, &id]).await
        },
        UserUpdate::Pwdhash(hash) => {
            client.execute("UPDATE users SET pwdhash=$1 where id=$2", &[&hash, &id]).await
        },
        UserUpdate::IsAdmin(is_admin) => {
            client.execute("UPDATE users SET is_admin=$1 WHERE id=$2", &[&is_admin, &id]).await
        }
    }.map_err(|e| {Box::new(e).into()}).map(|c| {Some(c)})
    
    //let changed = client.execute("UPDATE users SET id=$1, username=$2", params)
}

//FIXME: Fix error handling