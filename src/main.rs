
#[macro_use] extern crate rocket;
mod routes;
pub use crate::routes::api::lol::doit;

use std::{path::Path};
use rocket::{response};


#[get("/")]
fn index() -> response::Redirect {
  response::Redirect::to(uri!("/welcome"))
}

#[get("/welcome")]
async fn welcome() -> String{
  "Please login or register".to_string()
}


#[launch]
fn rocket() -> _ {
  rocket::build()
  .mount("/testing", routes::testing::get_routes())
  .mount("/", routes![index, welcome])
  .mount("/files", routes::files::get_routes())
}