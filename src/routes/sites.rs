use rocket::request::FlashMessage;
use rocket::{Route, fs::NamedFile, response::{Redirect, Flash}};
use std::path::Path;
use crate::slogic::{Admin, User, IUser};

pub fn get_routes() -> Vec<Route> {
    routes![welcome, user_dashboard, admin_dashboard, admin_dashboard_redir, admin_dashboard_prerr, user_dashboard_nocred]
}

#[get("/welcome")]
pub async fn welcome(flash: Option<FlashMessage<'_>>) -> Result<NamedFile, rocket::http::Status>{
    let path = Path::new("./client/sites/welcome.html");
    NamedFile::open(path).await.map_err(|_| {rocket::http::Status::InternalServerError}) 
}

#[get("/users/dashboard")]
pub async fn user_dashboard(user: User) -> Result<NamedFile, rocket::http::Status> {
    let path = Path::new("./client/sites/user_dashboard.html");
    NamedFile::open(path).await.map_err(|_| {rocket::http::Status::InternalServerError})
}

#[get("/users/dashboard", rank=2)]
async fn user_dashboard_nocred() -> Flash<Redirect> {
    Flash::error(Redirect::to(uri!("/welcome")), "Not logged in!")
}

#[get("/admin/dashboard")]
pub async fn admin_dashboard(admin: Admin) -> Result<NamedFile, Flash<Redirect>> {
    let path = Path::new("./client/sites/admin_dashboard.html");
    Ok(NamedFile::open(path).await.unwrap())
}

#[get("/admin/dashboard", rank=2)]
pub async fn admin_dashboard_prerr(user: User) -> Result<NamedFile, Flash<Redirect>> {
    let path = Path::new("./client/sites/notadmin.html");
    Ok(NamedFile::open(path).await.unwrap())
}

#[get("/admin/dashboard", rank=3)]
pub async fn admin_dashboard_redir() -> Flash<Redirect> {
    Flash::error(Redirect::to(uri!("/welcome")), "Not logged in!") 
}