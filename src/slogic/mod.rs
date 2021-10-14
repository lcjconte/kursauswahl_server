mod UserStructs;
pub mod Uservalidation;
pub use UserStructs::*;

use lazy_static::lazy_static;
use std::collections::HashMap;
use tokio::sync::RwLock;
use rocket::http::CookieJar;
use std::fmt;
use bcrypt;
use rand::{thread_rng, Rng};
use crate::db;
use std::{error::Error, fmt::Display};
use anyhow::{Context, Result};

lazy_static! {
    static ref ACTIVE_SESSIONS: RwLock<HashMap<u128, i32>> = RwLock::new(HashMap::new());
}

const STANDARD_COST: u32 = 6;

#[derive(Debug)]
pub struct TokenInvalid {

}
impl Display for TokenInvalid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TokenInvalid")
    }
}
impl Error for TokenInvalid {}

pub async fn extract_secret(jar: &CookieJar<'_>) -> Result<u128> {
    let secret_cookie = jar.get("user_id").ok_or(TokenInvalid{})?;
    let user_secret: u128 = secret_cookie.value().parse()?;
    Ok(user_secret)
}

pub async fn create_user(ruser: LoginUser<'_>) -> Result<()> {
    match db::get_user(ruser.uname).await? {
        Some(_) => {return Err(anyhow::Error::msg("Conflict"));},
        None => {}
    }
    let pwdhash = bcrypt::hash(ruser.pwd, STANDARD_COST).unwrap();
    db::add_user(&User::new( //?
        0,
        ruser.uname,
        &pwdhash,
        false,
    )).await
}

pub async fn update_user_pwd(user: &User, new_pwd: &str) -> Result<()>{
    db::update_user(user.id, db::UserUpdate::Pwdhash(bcrypt::hash(new_pwd, STANDARD_COST).unwrap())).await.map(|_| {})
}

pub async fn verify_user(jar: &CookieJar<'_>) -> Result<User> {
    let user_secret: u128 = extract_secret(jar).await?;
    let user_id = get_session(user_secret).await.ok_or(TokenInvalid{})?;
    let user = db::user_by_id(user_id).await?.ok_or(TokenInvalid{})?;
    Ok(user)
}

pub async fn create_session(ruser: LoginUser<'_>) -> Result<Option<u128>> {
    let cuser: User;
    match db::get_user(ruser.uname).await? {
        None => {return Ok(None)},
        Some(r) => {cuser = r;}
    }
    if !bcrypt::verify(ruser.pwd, &cuser.pwdhash).context("Verification error")? {
        return Ok(None);
    }
    let secret: u128 = thread_rng().gen();
    add_session(cuser.id(), secret).await;
    Ok(Some(secret))
}

pub async fn add_session(uid: i32, sid: u128) {
    ACTIVE_SESSIONS.write().await.insert(sid, uid);
}
pub async fn get_session(sid: u128) -> Option<i32> {
    ACTIVE_SESSIONS.read().await.get(&sid).map(|uid| {uid.clone()})
}
pub async fn remove_session(sid: u128) -> Option<i32> {
    ACTIVE_SESSIONS.write().await.remove(&sid)
}