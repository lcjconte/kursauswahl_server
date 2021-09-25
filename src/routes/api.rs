
use crate::data;
use crate::structures::User;
use rocket::{http::Status, serde::{Deserialize, json::Json}};
use bcrypt::{hash};

#[post("/createuser", data="<user>")]
async fn create_user(user: Json<User>) -> Status{
    Status::Accepted
}

#[post("/getsession", data="<user>")]
async fn getsession(user: Json<User>) -> Json<u128> {
    Json::from(1)
}
