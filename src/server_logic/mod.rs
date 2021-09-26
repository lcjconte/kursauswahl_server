
use rocket::{outcome::IntoOutcome, request::{FromRequest, Request}, serde::{Deserialize, Serialize}};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub pwdhash: String,
    pub is_admin: bool,
}

#[async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = TokenInvalid;
     
    async fn from_request(req: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        verify_user(req.cookies()).await.map_err(|_| {TokenInvalid{}}).or_forward(())
    }
}

use rocket::http::CookieJar;
use std::fmt;
use crate::db;
use std::{error::Error, fmt::Display};
use crate::active_sessions;

#[derive(Debug)]
pub struct TokenInvalid {

}
impl Display for TokenInvalid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TokenInvalid")
    }
}
impl Error for TokenInvalid {}

pub async fn verify_user(jar: &CookieJar<'_>) -> Result<User, Box<dyn Error>> {
    let secret_cookie = jar.get("user_id").ok_or(TokenInvalid{})?;
    let user_secret: u128 = secret_cookie.value().parse()?;
    let user_id = (*active_sessions.read().unwrap()).get(&user_secret).ok_or(TokenInvalid{})?.clone();
    let user = db::user_by_id(user_id).await?.ok_or(TokenInvalid{})?;
    Ok(user)
}