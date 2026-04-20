use crate::{
    models::r#match::MatchState,
    repositories::persistent_match::error::PersistentMatchRepositoryResult,
};
use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait PersistentMatchRepository: Send + Sync + Debug {
    async fn create_match(
        &self,
        white_player_id: i32,
        black_player_id: i32,
        ending_state: &MatchState,
    ) -> PersistentMatchRepositoryResult<i32>;
}
