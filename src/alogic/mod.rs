use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Selection {
    pub data: Vec<i32>
}

impl Selection {
    pub fn new(data: Vec<i32>) -> Selection{
        Selection {data}
    }
}

#[derive(Deserialize, Serialize)]
pub struct CourseInfo {
    pub id: i32,
    pub uname: String,
    pub fullname: String,
}