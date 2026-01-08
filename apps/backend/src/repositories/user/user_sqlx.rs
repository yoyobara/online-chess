use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::{
    models::user::UserData,
    repositories::user::{error::UserRepositoryResult, user::UserRepository, UserRepositoryError},
};

#[derive(Debug, Clone)]
pub struct SqlxUserRepository {
    pool: Pool<Postgres>,
}

impl SqlxUserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

fn sqlx_to_repo_error(err: sqlx::Error) -> UserRepositoryError {
    match err {
        sqlx::Error::RowNotFound => UserRepositoryError::UserNotFound,
        sqlx::Error::Database(db_err) if db_err.constraint().is_some() => {
            let violated_constraint = db_err.constraint().unwrap();

            UserRepositoryError::ConstraintViolation(violated_constraint.to_owned())
        }
        _ => UserRepositoryError::Db(err.into()),
    }
}

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn get_user(&self, id: i32) -> UserRepositoryResult<UserData> {
        sqlx::query_as!(UserData, "select * from users where id = $1", id)
            .fetch_one(&self.pool)
            .await
            .map_err(sqlx_to_repo_error)
    }

    async fn get_by_email(&self, email: String) -> UserRepositoryResult<UserData> {
        sqlx::query_as!(UserData, "select * from users where email = $1", email)
            .fetch_one(&self.pool)
            .await
            .map_err(sqlx_to_repo_error)
    }
    async fn create_user(
        &self,
        username: String,
        email: String,
        password_hash: String,
    ) -> UserRepositoryResult<i32> {
        sqlx::query_scalar!(
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id;",
            username,
            email,
            password_hash
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_to_repo_error)
    }
}
