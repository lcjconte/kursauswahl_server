use crate::active_sessions;
use crate::db;
use crate::server_logic::User;
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
    routes![create_user, getsession]
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
    if let Err(e) = db::add_user(User { //?
        id: 0,
        username: user.uname.to_string(),
        pwdhash: pwdhash,
        is_admin: false,
    })
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
    match bcrypt::verify(user.pwd, &cuser.pwdhash) {
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
    let mut wdata = active_sessions.write().unwrap();
    (*wdata).insert(secret, cuser.id);
    cookies.add(Cookie::new("user_id", secret.to_string()));
    Ok(Json(secret))
}

//TODO: /getuser
//TODO: Set Selection
//TODO: get Selection
//TODO: admin tools fetch selection