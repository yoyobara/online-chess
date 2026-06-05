use crate::{
    models::{chat::ChatMessage, r#match::MatchState},
    repositories::persistent_match::error::PersistentMatchRepositoryResult,
};
use async_trait::async_trait;
use rust_chess::core::chess_move::Move;
use std::fmt::Debug;

#[async_trait]
pub trait PersistentMatchRepository: Send + Sync + Debug {
    async fn create_match(
        &self,
        white_player_id: i32,
        black_player_id: i32,
        ending_state: &MatchState,
        moves: Vec<Move>,
        chat_messages: Vec<ChatMessage>,
    ) -> PersistentMatchRepositoryResult<i32>;
}
