use std::fmt::Debug;

use async_trait::async_trait;

use crate::{models::user::UserData, repositories::user::error::UserRepositoryResult};

#[async_trait]
pub trait UserRepository: Send + Sync + Debug {
    async fn get_user(&self, id: i32) -> UserRepositoryResult<UserData>;
    async fn get_by_email(&self, email: String) -> UserRepositoryResult<UserData>;
    async fn create_user(
        &self,
        username: String,
        email: String,
        password_hash: String,
    ) -> UserRepositoryResult<i32>;
}
