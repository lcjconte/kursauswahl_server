use serde::{Deserialize, Serialize};

pub trait IUser {
    fn id(&self) -> i32;
    fn username(&self) -> &String;
    fn pwdhash(&self) -> &String;
    fn is_admin(&self) -> bool;
    fn new(id: i32, username: &str, pwdhash: &str, is_admin: bool) -> Self;
}
#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub pwdhash: String,
    pub is_admin: bool,
}
impl IUser for User {
    fn id(&self) -> i32 {self.id}
    fn username(&self) -> &String {&self.username}
    fn pwdhash(&self) -> &String {&self.pwdhash}
    fn is_admin(&self) -> bool {self.is_admin}
    fn new(id: i32, username: &str, pwdhash: &str, is_admin: bool) -> Self {User {
        id, username: username.to_string(), pwdhash: pwdhash.to_string(), is_admin}}
}
pub struct  Admin { //Only use for routing
    pub user: User
}

#[derive(Deserialize)]
pub struct LoginUser<'a> {
    pub uname: &'a str,
    pub pwd: &'a str,
}