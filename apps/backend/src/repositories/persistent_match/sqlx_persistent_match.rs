use anyhow::anyhow;
use async_trait::async_trait;
use rust_chess::core::color::Color;
use sqlx::{Pool, Postgres};

use crate::{
    models::r#match::{MatchResult, MatchState},
    repositories::persistent_match::{
        error::{PersistentMatchRepositoryError, PersistentMatchRepositoryResult},
        persistent_match::PersistentMatchRepository,
    },
};

#[derive(Debug)]
pub struct SqlxPersistentMatchRepository {
    pool: Pool<Postgres>,
}

impl SqlxPersistentMatchRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "match_result_enum", rename_all = "snake_case")]
enum PersistentMatchResult {
    WhiteWon,
    BlackWon,
    Draw,
}

#[async_trait]
impl PersistentMatchRepository for SqlxPersistentMatchRepository {
    async fn create_match(
        &self,
        white_player_id: i32,
        black_player_id: i32,
        ending_state: &MatchState,
    ) -> PersistentMatchRepositoryResult<i32> {
        let result: PersistentMatchResult = match ending_state.match_result {
            Some(MatchResult::Win(Color::White)) => PersistentMatchResult::WhiteWon,
            Some(MatchResult::Win(Color::Black)) => PersistentMatchResult::BlackWon,
            Some(MatchResult::Draw) => PersistentMatchResult::Draw,
            None => return Err(anyhow!("no match result").into()),
        };

        let board_json = serde_json::to_string(&ending_state.board)?;

        sqlx::query_scalar!(
            "INSERT INTO matches (white_player_id, black_player_id, ending_board, move_count, match_result) VALUES ($1, $2, $3, $4, $5) RETURNING id;",
            white_player_id,
            black_player_id,
            serde_json::Value::String(board_json),
            ending_state.move_count,
            result as PersistentMatchResult
        )
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }
}

impl From<sqlx::Error> for PersistentMatchRepositoryError {
    fn from(err: sqlx::Error) -> Self {
        PersistentMatchRepositoryError::Unknown(err.into())
    }
}

impl From<serde_json::Error> for PersistentMatchRepositoryError {
    fn from(err: serde_json::Error) -> Self {
        PersistentMatchRepositoryError::Unknown(err.into())
    }
}
