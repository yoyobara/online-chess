use anyhow::Result;
use rust_chess::{
    board::{Board, EndgameState},
    core::{chess_move::Move as ChessMove, color::Color},
};

use crate::{
    models::r#match::{MatchResult, MatchState},
    utils::{
        pubsub::message::PubSubMessage,
        realtime::{
            client_communication::message::{PlayerMoveData, ServerMessage},
            RealtimeSession,
        },
    },
};

pub async fn handle_client_player_move(
    session: &mut RealtimeSession,
    move_data: PlayerMoveData,
) -> Result<()> {
    let mv = ChessMove::new(
        move_data.src_square,
        move_data.dest_square,
        move_data.captured_piece,
        None,
    );

    let mut match_state = session
        .app_state
        .match_repo
        .get_match_state(&session.match_id)
        .await?;

    let your_turn = match_state.move_count % 2
        == (if session.player_color == Color::White {
            0
        } else {
            1
        });

    let your_piece =
        match_state.board.get(mv.from).map(|p| p.piece_color) == Some(session.player_color);

    if your_turn
        && your_piece
        && match_state
            .board
            .get_legal_moves(mv.from)
            .unwrap()
            .contains(&mv)
    {
        match_state.board.apply_move(mv);
        match_state.move_count += 1;

        session
            .communicator
            .send(ServerMessage::MoveResult(true))
            .await?;

        let endgame_state = match_state
            .board
            .is_player_under_endgame_state(session.opponent_color);

        match_state.game_result = endgame_state.map(|state| match state {
            EndgameState::Checkmate => MatchResult::Win(session.player_color),
            EndgameState::Stalemate => MatchResult::Draw,
        });

        session
            .app_state
            .match_repo
            .update_match_state(&session.match_id, &match_state)
            .await?;

        session
            .pubsub
            .publish(
                &format!("match:{}", session.match_id),
                &PubSubMessage::PlayerMove(match_state),
            )
            .await?;
    } else {
        session
            .communicator
            .send(ServerMessage::MoveResult(false))
            .await?;
    }
    Ok(())
}
