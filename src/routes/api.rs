use crate::db;
use crate::slogic::{User, extract_secret, IUser, remove_session, add_session};
use bcrypt;
use rand::{thread_rng, Rng};
use rocket::http::{CookieJar, Cookie};
use rocket::{
    http::Status,
    serde::{json::Json, Deserialize},
};

const STANDARD_COST: u32 = 6;

#[derive(Deserialize)]
struct InUser<'a> {
    uname: &'a str,
    pwd: &'a str,
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![create_user, getsession, destroysession, get_user_info]
}

#[post("/createuser", data = "<user>")]
async fn create_user(user: Json<InUser<'_>>) -> Status {
    match db::get_user(user.uname).await {
        Err(e) => {
            eprintln!("{:?}", e);
            return Status::InternalServerError;
        }
        Ok(r) => {
            if let Some(_) = r {
                return Status::Conflict;
            }
        }
    }
    let pwdhash = bcrypt::hash(user.pwd, STANDARD_COST).unwrap();
    if let Err(e) = db::add_user(User::new( //?
        0,
        user.uname,
        &pwdhash,
        false,
    ))
    .await
    {
        eprintln!("{:?}", e);
        Status::InternalServerError
    } else {
        Status::Accepted
    }
}

#[post("/getsession", data = "<user>")]
async fn getsession(user: Json<InUser<'_>>, cookies: &CookieJar<'_>) -> Result<Json<u128>, Status> {
    //Get User then verify
    let cuser: User;
    match db::get_user(user.uname).await {
        Err(_) => return Err(Status::InternalServerError),
        Ok(r) => {
            if r.is_none() {
                return Err(Status::Forbidden);
            } else {
                cuser = r.unwrap();
            }
        }
    }
    match bcrypt::verify(user.pwd, cuser.pwdhash()) {
        Ok(r) => {
            if !r {
                return Err(Status::Forbidden);
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
            return Err(Status::InternalServerError);
        }
    }
    let secret: u128 = thread_rng().gen();
    add_session(cuser.id(), secret).await;
    cookies.add(Cookie::new("user_id", secret.to_string()));
    Ok(Json(secret))
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