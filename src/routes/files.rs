use rocket::Route;
use std::path::Path;
use rocket::fs::NamedFile;

pub fn get_routes() -> Vec<Route>{
    routes![welcometxt]
}

#[get("/welcome.txt")]
async fn welcometxt() -> Option<NamedFile>{
    let path = Path::new("./client/files/welcome.txt");
    NamedFile::open(path).await.ok()
}