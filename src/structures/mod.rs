use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub pwdhash: String,
    pub isAdmin: bool,
}