use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::{
    models::user::User,
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

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn get_user(&self, id: i32) -> UserRepositoryResult<User> {
        sqlx::query_as!(User, "select * from users where id = $1", id)
            .fetch_one(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn get_by_email(&self, email: String) -> UserRepositoryResult<User> {
        sqlx::query_as!(User, "select * from users where email = $1", email)
            .fetch_one(&self.pool)
            .await
            .map_err(Into::into)
    }
    async fn create_user(
        &self,
        username: String,
        email: String,
        password_hash: String,
        initial_rank: i32,
    ) -> UserRepositoryResult<i32> {
        sqlx::query_scalar!(
            "INSERT INTO users (username, email, password_hash, rank) VALUES ($1, $2, $3, $4) RETURNING id;",
            username,
            email,
            password_hash,
            initial_rank
        )
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }
}

impl From<sqlx::Error> for UserRepositoryError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => UserRepositoryError::UserNotFound,
            sqlx::Error::Database(db_err) if db_err.constraint().is_some() => {
                let violated_constraint = db_err.constraint().unwrap();

                UserRepositoryError::ConstraintViolation(violated_constraint.to_owned())
            }
            _ => UserRepositoryError::Db(err.into()),
        }
    }
}
