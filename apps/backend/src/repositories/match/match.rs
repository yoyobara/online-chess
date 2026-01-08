use std::fmt::Debug;

use async_trait::async_trait;

use crate::repositories::r#match::error::MatchRepositoryResult;

#[async_trait]
pub trait MatchRepository: Send + Sync + Debug {
    async fn pop_matchmaking_player(&self) -> MatchRepositoryResult<Option<i32>>;
    async fn push_matchmaking_player(&self, player_id: i32) -> MatchRepositoryResult<()>;

    async fn register_match(&self, player_1_id: i32, player_2_id: i32)
        -> MatchRepositoryResult<()>;
    async fn is_player_in_match(
        &self,
        player_id: i32,
        match_id: &str,
    ) -> MatchRepositoryResult<bool>;
}
