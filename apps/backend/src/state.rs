use std::sync::Arc;

use crate::{configs::Config, repositories::user::UserRepository};

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Arc<Config>,
    pub user_repo: Arc<dyn UserRepository>,
}

impl AppState {
    pub async fn new(config: Arc<Config>, user_repo: Arc<dyn UserRepository>) -> Self {
        Self { config, user_repo }
    }
}
