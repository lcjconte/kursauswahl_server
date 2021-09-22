
#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> &'static str {
  "This is my Rocket Demo app"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}