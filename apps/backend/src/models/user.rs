use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::prelude::*;

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub rank: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Debug)]
pub struct UserData {
    username: String,
    email: String,
    rank: i32,
}

impl From<User> for UserData {
    fn from(value: User) -> Self {
        Self {
            username: value.username,
            rank: value.rank,
            email: value.email,
        }
    }
}
