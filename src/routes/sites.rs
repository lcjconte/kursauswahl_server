

use rocket::http::CookieJar;
use rocket::{Route, fs::NamedFile};
use std::path::Path;
use crate::{localization, rocket};
use crate::{utils::verify_user};

pub fn get_routes() -> Vec<Route> {
    routes![welcome, dashboard]
}

#[get("/welcome")]
pub async fn welcome() -> Result<NamedFile, rocket::http::Status>{
    let path = Path::new("./client/sites/welcome.html");
    NamedFile::open(path).await.map_err(|_| {rocket::http::Status::InternalServerError}) 
}

#[get("/users/dashboard")]
pub async fn dashboard(jar: &CookieJar<'_>) -> Result<NamedFile, rocket::http::Status> {
    let user = verify_user(jar).await.map_err(|_| {rocket::http::Status::Forbidden})?;
    let path = Path::new("./client/sites/user_dashboard.html");
    NamedFile::open(path).await.map_err(|_| {rocket::http::Status::InternalServerError})
}