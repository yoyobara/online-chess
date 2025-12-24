use std::fmt::Debug;

use async_trait::async_trait;

use crate::{models::user::UserData, repositories::user::UserRepositoryError};

#[async_trait]
pub trait UserRepository: Send + Sync + Debug {
    async fn get_user(&self, id: i32) -> Result<UserData, UserRepositoryError>;
    async fn get_by_email(&self, email: String) -> Result<UserData, UserRepositoryError>;
    async fn create_user(
        &self,
        username: String,
        email: String,
        password_hash: String,
    ) -> Result<i32, UserRepositoryError>;
}
