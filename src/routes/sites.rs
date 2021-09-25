

use rocket::{Route, fs::NamedFile};
use std::path::Path;
use rocket::error::Error;
use crate::{localization, rocket};

pub fn get_routes() -> Vec<Route> {
    routes![welcome]
}

#[get("/welcome")]
pub async fn welcome() -> Result<NamedFile, rocket::http::Status>{
    let path = Path::new("./client/sites/welcome.html");
    NamedFile::open(path).await.map_err(|e| {rocket::http::Status::InternalServerError}) 
}