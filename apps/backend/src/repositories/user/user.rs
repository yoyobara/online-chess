use std::fmt::Debug;

use async_trait::async_trait;

use crate::{
    models::{r#match::MatchResult, user::User},
    repositories::user::error::UserRepositoryResult,
};

#[async_trait]
pub trait UserRepository: Send + Sync + Debug {
    async fn get_user(&self, id: i32) -> UserRepositoryResult<User>;
    async fn get_by_email(&self, email: String) -> UserRepositoryResult<User>;
    async fn create_user(
        &self,
        username: String,
        email: String,
        password_hash: String,
        initial_rank: i32,
    ) -> UserRepositoryResult<i32>;
    async fn update_users_ranks_elo(
        &self,
        white_player_id: i32,
        black_player_id: i32,
        result: MatchResult,
    ) -> UserRepositoryResult<()>;
}
