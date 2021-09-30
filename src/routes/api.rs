use crate::db;
use crate::slogic::{IUser, LoginUser, User, create_session, create_user, extract_secret, remove_session};
use rocket::http::{CookieJar, Cookie};
use rocket::{
    http::Status,
    serde::{json::Json, Deserialize},
};

pub fn get_routes() -> Vec<rocket::Route> {
    routes![create_user_h, getsession, destroysession, get_user_info]
}

#[post("/createuser", data = "<user>")]
async fn create_user_h(user: Json<LoginUser<'_>>) -> Status {
    match create_user(user.0).await {
        Ok(()) => Status::Accepted,
        Err(e) => if e.to_string() == "conflict" { Status::Conflict} else {eprintln!("{:?}", e);Status::InternalServerError}
    }
}

#[post("/getsession", data = "<user>")]
async fn getsession(user: Json<LoginUser<'_>>, cookies: &CookieJar<'_>) -> Result<Json<u128>, Status> {
    //Get User then verify
    let secret = create_session(user.0).await.map_err(|e| {eprintln!("{:?}", e);Status::InternalServerError})?;
    match secret {
        None => Err(Status::BadRequest),
        Some(s) => {
            cookies.add(Cookie::new("user_id", s.to_string()));
            Ok(Json(s))
        }
    }
}

#[post("/destroysession")]
async fn destroysession(jar: &CookieJar<'_>) -> Result<Status, Status> {
    let sid = extract_secret(jar).await.map_err(|_| {Status::BadRequest})?;
    match remove_session(sid).await {
        None => {Err(Status::BadRequest)},
        Some(_) => {jar.remove(Cookie::named("user_id"));Ok(Status::Accepted)}
    }
}

#[post("/user_info")]
async fn get_user_info(user: User) -> Result<Json<User>, Status> {
    Ok(Json(user))
}

//TODO: Set Selection___
//TODO: get Selection___
//TODO: admin tools fetch selection