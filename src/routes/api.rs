use crate::alogic::Selection;
use crate::db;
use crate::slogic::{LoginUser, User, create_session, create_user, extract_secret, remove_session};
use rocket::http::{CookieJar, Cookie, Status};
use rocket::serde::{json::Json, Deserialize};


pub fn get_routes() -> Vec<rocket::Route> {
    routes![rcreate_user, rgetsession, rdestroysession, get_user_info, rget_selection]
}

type ApiResponse<R, E> = Result<Json<R>, E>;

#[post("/createuser", data = "<user>")]
async fn rcreate_user(user: Json<LoginUser<'_>>) -> ApiResponse<(), Status> {
    create_user(user.0).await.map(|_|{Json(())}).map_err(|e| {
        if e.to_string() == "conflict" { Status::Conflict } else {eprintln!("{:?}", e);Status::InternalServerError}
    })
}

#[post("/getsession", data = "<user>")]
async fn rgetsession(user: Json<LoginUser<'_>>, cookies: &CookieJar<'_>) -> ApiResponse<u128, Status> {
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
async fn rdestroysession(jar: &CookieJar<'_>) -> ApiResponse<u128, Status> {
    let sid = extract_secret(jar).await.map_err(|_| {Status::BadRequest})?;
    match remove_session(sid).await {
        None => {Err(Status::BadRequest)},
        Some(_) => {jar.remove(Cookie::named("user_id"));Ok(Json(sid))}
    }
}

#[post("/user_info")]
async fn get_user_info(user: User) -> ApiResponse<User, Status> {
    Ok(Json(user))
}

#[get("/selection")]
async fn rget_selection(user: Option<User>) -> ApiResponse<Selection, Status> {
    if user.is_none(){
        return Err(Status::Forbidden);
    }
    db::get_selection(&user.unwrap()).await.map_err(|_| {Status::BadRequest}).map(|e| {Json(e)})
}

#[put("/selection", data="<sel>")]
async fn rupdate_selection(user: User, sel: Json<Selection>) -> ApiResponse<(), Status>{
    Ok(Json(()))
}

//TODO: Set Selection___
//TODO: get Selection___
//TODO: admin tools fetch selection