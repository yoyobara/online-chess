use std::fmt::Debug;

use async_trait::async_trait;

use crate::{models::user::User, repositories::user::error::UserRepositoryResult};

#[async_trait]
pub trait UserRepository: Send + Sync + Debug {
    async fn get_user(&self, id: i32) -> UserRepositoryResult<User>;
    async fn get_by_email(&self, email: String) -> UserRepositoryResult<User>;
    async fn create_user(
        &self,
        username: String,
        email: String,
        password_hash: String,
    ) -> UserRepositoryResult<i32>;
}
