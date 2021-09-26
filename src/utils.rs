use rocket::http::CookieJar;
use std::fmt;
use crate::{db, structures::User};
use std::{error::Error, fmt::Display};
use crate::data;

#[derive(Debug)]
struct TokenInvalid {

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
    let user_id = (*data.read().unwrap()).get(&user_secret).ok_or(TokenInvalid{})?.clone();
    let user = db::user_by_id(user_id).await?.ok_or(TokenInvalid{})?;
    Ok(user)
}