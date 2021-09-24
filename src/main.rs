
#[macro_use] extern crate rocket;

use std::{path::Path, time::Duration};
use tokio::time::sleep;
use rocket::{fs::NamedFile, response};
use tokio_postgres::{NoTls};
use std::env;
use dotenv::dotenv;

#[get("/")]
fn index() -> response::Redirect {
  response::Redirect::to(uri!("/welcome"))
}

#[get("/welcome")]
async fn welcome() -> Option<NamedFile>{
  let path = Path::new("./client/files/welcome.txt");
  NamedFile::open(path).await.ok()
}

#[get("/wait10")]
async fn wait10() -> &'static str{
  sleep(Duration::new(10, 0)).await;
  "Finished"
}

#[get("/course")]
async fn course() -> response::status::Accepted<String> {
  dotenv().ok();
  let connurl = env::var("DATABASE_URL").unwrap();
  let (client, connection) =
    tokio_postgres::connect(&connurl, NoTls).await.unwrap();
  tokio::spawn(async move {
    if let Err(e) = connection.await {
      eprintln!("connection error: {}", e);
    }
  });
  let rows = client
    .query("SELECT * FROM testable", &[])
    .await.unwrap();
  let value: i32 = rows[0].get(0);
  response::status::Accepted(Some(value.to_string()))
}

#[launch]
fn rocket() -> _ {
  rocket::build()
  .mount("/testing", routes![wait10, course])
  .mount("/", routes![index, welcome])
}