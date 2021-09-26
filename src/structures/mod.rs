
use rocket::{serde::{Deserialize, Serialize}};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub pwdhash: String,
    pub is_admin: bool,
}