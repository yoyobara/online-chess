use std::fmt::Debug;

use async_trait::async_trait;
use rust_chess::board::Board;

use crate::{
    models::r#match::{MatchPlayers, MatchState},
    repositories::ephemeral_match::error::EphemeralMatchRepositoryResult,
};

#[async_trait]
pub trait EphemeralMatchRepository: Send + Sync + Debug {
    async fn pop_matchmaking_player(&self) -> EphemeralMatchRepositoryResult<Option<i32>>;
    async fn push_matchmaking_player(&self, player_id: i32) -> EphemeralMatchRepositoryResult<()>;

    async fn register_match(
        &self,
        white_player_id: i32,
        black_player_id: i32,
        starting_board: Board,
    ) -> EphemeralMatchRepositoryResult<String>;

    async fn is_player_in_match(
        &self,
        player_id: i32,
        match_id: &str,
    ) -> EphemeralMatchRepositoryResult<bool>;

    async fn get_match_state(&self, match_id: &str) -> EphemeralMatchRepositoryResult<MatchState>;
    async fn update_match_state(
        &self,
        match_id: &str,
        new_state: &MatchState,
    ) -> EphemeralMatchRepositoryResult<()>;

    async fn get_players(&self, match_id: &str) -> EphemeralMatchRepositoryResult<MatchPlayers>;
}
