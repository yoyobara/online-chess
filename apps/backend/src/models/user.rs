use chrono::NaiveDateTime;
use sqlx::prelude::*;

#[derive(FromRow)]
pub struct UserData {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub rank: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
