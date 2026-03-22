use anyhow::Result;
use rust_chess::{
    board::EndgameState,
    core::{chess_move::Move, color::Color},
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

fn allowed_to_move(session: &RealtimeSession, match_state: &MatchState, mv: Move) -> bool {
    let your_turn = match_state.move_count % 2
        == (if session.player_color == Color::White {
            0
        } else {
            1
        });

    let your_piece =
        match_state.board.get(mv.from).map(|p| p.piece_color) == Some(session.player_color);

    your_turn
        && your_piece
        && match_state
            .board
            .get_legal_moves(mv.from)
            .unwrap()
            .contains(&mv)
}

fn get_match_result(session: &RealtimeSession, match_state: &MatchState) -> Option<MatchResult> {
    let endgame_state = match_state
        .board
        .is_player_under_endgame_state(session.opponent_color);

    endgame_state.map(|state| match state {
        EndgameState::Checkmate => MatchResult::Win(session.player_color),
        EndgameState::Stalemate => MatchResult::Draw,
    })
}

async fn finalize_match(session: &mut RealtimeSession, match_state: &MatchState) -> Result<()> {
    session
        .app_state
        .persistent_match_repo
        .create_match(session.player_id, session.opponent_id, &match_state)
        .await?;

    session
        .app_state
        .ephemeral_match_repo
        .finalize_match(&session.match_id, session.player_id, session.opponent_id)
        .await?;

    Ok(())
}

pub async fn handle_client_player_move(
    session: &mut RealtimeSession,
    move_data: PlayerMoveData,
) -> Result<()> {
    let mv = Move::new(
        move_data.src_square,
        move_data.dest_square,
        move_data.promotion,
        move_data.move_type,
    );

    let mut match_state = session
        .app_state
        .ephemeral_match_repo
        .get_match_state(&session.match_id)
        .await?;

    if allowed_to_move(session, &match_state, mv) {
        match_state.board.apply_move(mv);
        match_state.move_count += 1;

        session
            .communicator
            .send(ServerMessage::MoveResult(true))
            .await?;

        match_state.match_result = get_match_result(session, &match_state);

        if match_state.match_result.is_some() {
            finalize_match(session, &match_state).await?;
        } else {
            session
                .app_state
                .ephemeral_match_repo
                .update_match_state(&session.match_id, &match_state)
                .await?;
        }

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
