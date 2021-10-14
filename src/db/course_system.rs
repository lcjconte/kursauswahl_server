use super::{connect, DBError};
use anyhow::{Result, Context};


pub async fn get_courses() -> Result<Vec<i32>>{
    let client = connect().await?;
    Ok(client.query("SELECT id FROM courses", &[]).await?.iter().map(|r| {r.get("id")}).collect())
}


