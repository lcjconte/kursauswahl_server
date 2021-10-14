use super::{connect, DBError};
use anyhow::{Result, Context};
use crate::slogic::{User, IUser};

pub async fn get_user(uname: &str) -> Result<Option<User>> {  //DOUBLE!!
    let client = connect().await?;
    let rows = client
        .query("SELECT * FROM users WHERE username = $1", &[&uname])
        .await.context("Query Error")?;
    if rows.len() == 0{
        Ok(None)
    }
    else {
        Ok(Some(User::new(rows[0].get("id"),rows[0].get("username"),rows[0].get("pwdhash"),rows[0].get("isAdmin"))))
    }
}

pub async fn user_by_id(id: i32) -> Result<Option<User>> {
    let client= connect().await?;
    let rows = client
        .query("SELECT * FROM users WHERE id = $1", &[&id])
        .await.context("Query Error")?;
    if rows.len() == 0{
        Ok(None)
    }
    else {
        Ok(Some(User::new(rows[0].get("id"),rows[0].get("username"),rows[0].get("pwdhash"),rows[0].get("isAdmin"))))
    }
}

pub async fn add_user(user: &User) -> Result<()>{
    let client = connect().await?;
    let changed = client.execute("INSERT INTO users VALUES (DEFAULT, $1, $2, $3)", &[user.username(), user.pwdhash(), &user.is_admin()]).await
        .context("Execute error")?;
    if changed==1 { Ok(()) } else {Err(DBError::Conflict.into())}
}

pub enum UserUpdate {
    Username(String),
    Pwdhash(String),
    IsAdmin(bool),
}

pub async fn update_user(id: i32, update: UserUpdate) -> Result<Option<u64>> {
    let client = connect().await?;
    if user_by_id(id).await?.is_none() {
        return Ok(None);
    }
    match update {
        UserUpdate::Username(name) => {
            if let Some(_conflict) = get_user(&name).await? {
                return Err(DBError::Conflict.into());
            }
            client.execute("UPDATE users SET username=$1 where id=$2", &[&name, &id]).await
        },
        UserUpdate::Pwdhash(hash) => {
            client.execute("UPDATE users SET pwdhash=$1 where id=$2", &[&hash, &id]).await
        },
        UserUpdate::IsAdmin(is_admin) => {
            client.execute("UPDATE users SET is_admin=$1 WHERE id=$2", &[&is_admin, &id]).await
        }
    }.map_err(|e| {Box::new(e).into()}).map(|c| {Some(c)})
    
    //let changed = client.execute("UPDATE users SET id=$1, username=$2", params)
}