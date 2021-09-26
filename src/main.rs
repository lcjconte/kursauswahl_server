
#[macro_use] extern crate rocket;
mod routes;
mod db;
mod localization;
mod server_logic;
mod app_logic;

use std::{collections::HashMap, path::Path, path::PathBuf};
use rocket::{fs::NamedFile, response};
use std::sync::RwLock;
use lazy_static::lazy_static;

lazy_static! {
    static ref active_sessions: RwLock<HashMap<u128, i32>> = RwLock::new(HashMap::new());
}

#[get("/")]
fn index() -> response::Redirect {
  response::Redirect::to(uri!("/welcome"))
}

#[get("/scripts/<path..>")]
async fn scripts(path: PathBuf) -> NamedFile{
    let path = Path::new("./client/scripts/").join(path);
    NamedFile::open(path).await.unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/testing", routes::testing::get_routes())
    .mount("/", routes![index/*, welcome*/,scripts])
    .mount("/", routes::sites::get_routes())
    .mount("/files", routes::files::get_routes())
    .mount("/api", routes::api::get_routes())
}