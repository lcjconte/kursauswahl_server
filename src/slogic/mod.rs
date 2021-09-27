
use rocket::{outcome::IntoOutcome, request::{FromRequest, Request}, serde::{Deserialize, Serialize}};
use lazy_static::lazy_static;
use std::collections::HashMap;
use tokio::sync::RwLock;
use rocket::http::CookieJar;
use std::fmt;
use crate::db;
use std::{error::Error, fmt::Display};

lazy_static! {
    static ref ACTIVE_SESSIONS: RwLock<HashMap<u128, i32>> = RwLock::new(HashMap::new());
}
//region
pub trait IUser {
    fn id(&self) -> i32;
    fn username(&self) -> &String;
    fn pwdhash(&self) -> &String;
    fn is_admin(&self) -> bool;
    fn new(id: i32, username: &str, pwdhash: &str, is_admin: bool) -> Self;
}
#[derive(Deserialize, Serialize)]
pub struct User {
    id: i32,
    username: String,
    pwdhash: String,
    is_admin: bool,
}
impl IUser for User {
    fn id(&self) -> i32 {self.id}
    fn username(&self) -> &String {&self.username}
    fn pwdhash(&self) -> &String {&self.pwdhash}
    fn is_admin(&self) -> bool {self.is_admin}
    fn new(id: i32, username: &str, pwdhash: &str, is_admin: bool) -> Self {User {
        id, username: username.to_string(), pwdhash: pwdhash.to_string(), is_admin}}
}
pub struct  Admin { //Only use for routing
    user: User
}
impl IUser for Admin {
    fn id(&self) -> i32 {self.user.id}
    fn username(&self) -> &String {&self.user.username}
    fn pwdhash(&self) -> &String {&self.user.pwdhash}
    fn is_admin(&self) -> bool {self.user.is_admin}
    fn new(id: i32, username: &str, pwdhash: &str, is_admin: bool) -> Self {Admin {user: User {
        id, username: username.to_string(), pwdhash: pwdhash.to_string(), is_admin}}}
}
//endregion

#[async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = TokenInvalid;
     
    async fn from_request(req: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        verify_user(req.cookies()).await.map_err(|_| {TokenInvalid{}}).or_forward(())
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = TokenInvalid;
     
    async fn from_request(req: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let user = verify_user(req.cookies()).await;
        match user {
            Err(e) => Err(TokenInvalid{}),
            Ok(u) => if u.is_admin {
                Ok(Admin::new(u.id, &u.username, &u.pwdhash, u.is_admin))
            } else {
                Err(TokenInvalid{})
            }
        }.or_forward(())
    }
}

#[derive(Debug)]
pub struct TokenInvalid {

}
impl Display for TokenInvalid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TokenInvalid")
    }
}
impl Error for TokenInvalid {}

pub async fn extract_secret(jar: &CookieJar<'_>) -> Result<u128, Box<dyn Error>> {
    let secret_cookie = jar.get("user_id").ok_or(TokenInvalid{})?;
    let user_secret: u128 = secret_cookie.value().parse()?;
    Ok(user_secret)
}

pub async fn verify_user(jar: &CookieJar<'_>) -> Result<User, Box<dyn Error>> {
    let user_secret: u128 = extract_secret(jar).await?;
    let user_id = get_session(user_secret).await.ok_or(TokenInvalid{})?;
    let user = db::user_by_id(user_id).await?.ok_or(TokenInvalid{})?;
    Ok(user)
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