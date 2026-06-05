use anyhow::anyhow;
use async_trait::async_trait;
use rust_chess::core::{
    chess_move::{Move, MoveType},
    color::Color,
    piece::PieceType,
};
use sqlx::{Pool, Postgres};

use crate::{
    models::{
        chat::ChatMessage,
        r#match::{MatchResult, MatchState},
    },
    repositories::persistent_match::{
        error::{PersistentMatchRepositoryError, PersistentMatchRepositoryResult},
        persistent_match::PersistentMatchRepository,
    },
};

#[derive(sqlx::Type)]
#[sqlx(type_name = "match_result_enum", rename_all = "snake_case")]
enum PersistentMatchResult {
    WhiteWon,
    BlackWon,
    Draw,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "move_type_enum", rename_all = "snake_case")]
enum PersistentMoveType {
    Quiet,
    Capture,
    EnPassant,
    QueensideCastling,
    KingsideCastling,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "piece_type_enum", rename_all = "snake_case")]
enum PersistentPieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug)]
pub struct SqlxPersistentMatchRepository {
    pool: Pool<Postgres>,
}

impl SqlxPersistentMatchRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PersistentMatchRepository for SqlxPersistentMatchRepository {
    async fn create_match(
        &self,
        white_player_id: i32,
        black_player_id: i32,
        ending_state: &MatchState,
        moves: Vec<Move>,
        chat_messages: Vec<ChatMessage>,
    ) -> PersistentMatchRepositoryResult<i32> {
        let board_json = serde_json::to_string(&ending_state.board)?;
        let result: PersistentMatchResult = ending_state
            .match_result
            .ok_or(anyhow!("game is not over!"))?
            .into();

        let mut tx = self.pool.begin().await?;

        let match_id = sqlx::query_scalar!(
            "INSERT INTO matches (white_player_id, black_player_id, ending_board, match_result) VALUES ($1, $2, $3, $4) RETURNING id;",
            white_player_id,
            black_player_id,
            serde_json::Value::String(board_json),
            result as PersistentMatchResult
        )
        .fetch_one(&mut *tx)
        .await?;

        for (i, mv) in moves.iter().enumerate() {
            let move_type: PersistentMoveType = mv.move_type.into();
            let promotion: Option<PersistentPieceType> = mv.promotion.map(PieceType::into);
            sqlx::query!(
                "INSERT INTO moves (match_id, move_order, from_square, to_square, move_type, promotion) VALUES ($1, $2, $3, $4, $5, $6);",
                match_id,
                i as i32,
                mv.from as i32,
                mv.to as i32,
                move_type as PersistentMoveType,
                promotion as Option<PersistentPieceType>
            ).execute(&mut *tx).await?;
        }

        for msg in chat_messages {
            let msg_uuid = uuid::Uuid::parse_str(&msg.id).map_err(|e| anyhow!(e))?;
            sqlx::query!(
                "INSERT INTO chat_messages (id, match_id, author_id, content) VALUES ($1, $2, $3, $4);",
                msg_uuid,
                match_id,
                msg.author_id,
                msg.content
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(match_id)
    }
}

impl From<MatchResult> for PersistentMatchResult {
    fn from(result: MatchResult) -> Self {
        match result {
            MatchResult::Win(Color::White) => PersistentMatchResult::WhiteWon,
            MatchResult::Win(Color::Black) => PersistentMatchResult::BlackWon,
            MatchResult::Draw => PersistentMatchResult::Draw,
        }
    }
}

impl From<MoveType> for PersistentMoveType {
    fn from(move_type: MoveType) -> Self {
        match move_type {
            MoveType::Capture => Self::Capture,
            MoveType::EnPassant => Self::EnPassant,
            MoveType::KingsideCastling => Self::KingsideCastling,
            MoveType::QueensideCastling => Self::QueensideCastling,
            MoveType::Quiet => Self::Quiet,
        }
    }
}

impl From<PieceType> for PersistentPieceType {
    fn from(piece_type: PieceType) -> Self {
        match piece_type {
            PieceType::Bishop => Self::Bishop,
            PieceType::King => Self::King,
            PieceType::Knight => Self::Knight,
            PieceType::Pawn => Self::Pawn,
            PieceType::Queen => Self::Queen,
            PieceType::Rook => Self::Rook,
        }
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
