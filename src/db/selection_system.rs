use anyhow::{Result, Context};

use crate::slogic::User;
use crate::alogic::{Selection};

use super::connect;


pub async fn get_selection(user: &User) -> Result<Selection>{
    let client = connect().await?;
    let res = client.query("SELECT course FROM choices WHERE made_by = $1", &[&user.id]).await?;
    let mut courses: Vec<i32> = vec![];
    for row in res {
        courses.push(row.get("course"));
    }
    Ok(Selection::new(courses))
}

pub async fn del_selection(user: User) -> Result<()>{
    let client = connect().await?;
    client.execute("DELETE FROM choices WHERE made_by = $1", &[&user.id]).await.map_err(|e| {e.into()}).map(|_|{})
}

pub async fn set_selection(user: User, selection: &Selection) {

}
