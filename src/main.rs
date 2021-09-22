
#[macro_use] extern crate rocket;

use std::path::Path;
use rocket::{response, fs::NamedFile};

#[get("/")]
fn index() -> response::Redirect {
  response::Redirect::to(uri!("/welcome"))
}

#[get("/welcome")]
async fn welcome() -> Option<NamedFile>{
  let path = Path::new("./client/files/welcome.txt");
  NamedFile::open(path).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, welcome])
}