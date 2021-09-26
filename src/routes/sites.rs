

use rocket::http::CookieJar;
use rocket::{Route, fs::NamedFile};
use std::path::Path;
use crate::server_logic::{User, verify_user};

pub fn get_routes() -> Vec<Route> {
    routes![welcome, user_dashboard, admin_dashboard, admin_dashboard2]
}

#[get("/welcome")]
pub async fn welcome() -> Result<NamedFile, rocket::http::Status>{
    let path = Path::new("./client/sites/welcome.html");
    NamedFile::open(path).await.map_err(|_| {rocket::http::Status::InternalServerError}) 
}

#[get("/users/dashboard")]
pub async fn user_dashboard(jar: &CookieJar<'_>) -> Result<NamedFile, rocket::http::Status> {
    let _user = verify_user(jar).await.map_err(|_| {rocket::http::Status::Forbidden})?;
    let path = Path::new("./client/sites/user_dashboard.html");
    NamedFile::open(path).await.map_err(|_| {rocket::http::Status::InternalServerError})
}

#[get("/admin/dashboard")]
pub async fn admin_dashboard(jar: &CookieJar<'_>) -> Result<NamedFile, rocket::http::Status> {
    let user = verify_user(jar).await.map_err(|_| {rocket::http::Status::Forbidden})?;
    if !user.is_admin {return Err(rocket::http::Status::Forbidden);}
    let path = Path::new("./client/sites/admin_dashboard.html");
    NamedFile::open(path).await.map_err(|_| {rocket::http::Status::InternalServerError})
}
#[get("/admin/dashboard2")]
pub async fn admin_dashboard2(user: User) -> Result<NamedFile, rocket::http::Status> {
    if !user.is_admin {return Err(rocket::http::Status::Forbidden);}
    let path = Path::new("./client/sites/admin_dashboard.html");
    NamedFile::open(path).await.map_err(|_| {rocket::http::Status::InternalServerError})
}