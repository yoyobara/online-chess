use async_trait::async_trait;
use rust_chess::core::color::Color;
use sqlx::{Pool, Postgres};

use crate::{
    models::{r#match::MatchResult, user::User},
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

    async fn update_users_ranks_elo(
        &self,
        white_player_id: i32,
        black_player_id: i32,
        result: MatchResult,
    ) -> UserRepositoryResult<()> {
        let result_str = match result {
            MatchResult::Win(Color::White) => "white_win",
            MatchResult::Win(Color::Black) => "black_win",
            MatchResult::Draw => "draw",
        };

        sqlx::query!(
            "
            WITH current AS (
                SELECT id, rank
                FROM users
                WHERE id IN ($1, $2)
                FOR UPDATE
            )
            UPDATE users u
            SET rank = u.rank + 32 * (
                CASE 
                    WHEN u.id = $1 AND $3 = 'white_win' THEN 1
                    WHEN u.id = $2 AND $3 = 'black_win' THEN 1
                    WHEN $3 = 'draw' THEN 0.5
                    ELSE 0
                END
                -
                (
                    1.0 / (
                        1 + POWER(10,
                            (
                                (SELECT c.rank FROM current c
                                WHERE c.id = CASE 
                                    WHEN u.id = $1 THEN $2
                                    ELSE $1
                                END)
                                - u.rank
                            ) / 400.0
                        )
                    )
                )
            )
            WHERE u.id IN ($1, $2);
            ",
            white_player_id,
            black_player_id,
            result_str
        )
        .execute(&self.pool)
        .await?;

        Ok(())
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
