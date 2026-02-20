use std::fmt::Debug;

use async_trait::async_trait;

use crate::{
    models::r#match::{MatchPlayers, MatchState},
    repositories::r#match::error::MatchRepositoryResult,
};

#[async_trait]
pub trait MatchRepository: Send + Sync + Debug {
    async fn pop_matchmaking_player(&self) -> MatchRepositoryResult<Option<i32>>;
    async fn push_matchmaking_player(&self, player_id: i32) -> MatchRepositoryResult<()>;

    async fn register_match(
        &self,
        white_player_id: i32,
        black_player_id: i32,
    ) -> MatchRepositoryResult<String>;

    async fn is_player_in_match(
        &self,
        player_id: i32,
        match_id: &str,
    ) -> MatchRepositoryResult<bool>;

    async fn get_match_state(&self, match_id: &str) -> MatchRepositoryResult<MatchState>;
    async fn update_match_state(
        &self,
        match_id: &str,
        new_state: &MatchState,
    ) -> MatchRepositoryResult<()>;

    async fn get_players(&self, match_id: &str) -> MatchRepositoryResult<MatchPlayers>;
}
