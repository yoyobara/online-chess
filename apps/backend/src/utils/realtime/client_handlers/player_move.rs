use anyhow::Result;
use rust_chess::{board::EndgameState, core::chess_move::Move};

use crate::{
    models::r#match::{MatchResult, MatchState},
    utils::{
        pubsub::message::PubSubMessage,
        realtime::{client_communication::message::ServerMessage, RealtimeSession},
    },
};

fn allowed_to_move(session: &RealtimeSession, match_state: &MatchState, mv: Move) -> bool {
    let your_turn = match_state.current_turn == session.player_color;
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

async fn finalize_match(session: &mut RealtimeSession) -> Result<()> {
    let (players, state, moves, chat) = session
        .app_state
        .ephemeral_match_repo
        .finalize_match(&session.match_id)
        .await?;

    session
        .app_state
        .persistent_match_repo
        .create_match(
            players.white_player_id,
            players.black_player_id,
            &state,
            moves,
            chat,
        )
        .await?;

    session
        .app_state
        .user_repo
        .update_users_ranks_elo(
            players.white_player_id,
            players.black_player_id,
            state.match_result.unwrap(),
        )
        .await?;

    Ok(())
}

pub async fn handle_client_player_move(session: &mut RealtimeSession, mv: Move) -> Result<()> {
    let mut match_state = session
        .app_state
        .ephemeral_match_repo
        .get_match_state(&session.match_id)
        .await?;

    if allowed_to_move(session, &match_state, mv) {
        match_state.board.apply_move(mv);
        match_state.current_turn = !match_state.current_turn;
        match_state.match_result = get_match_result(session, &match_state);

        session
            .communicator
            .send(ServerMessage::MoveResult(true))
            .await?;

        session
            .app_state
            .ephemeral_match_repo
            .update_match_state(&session.match_id, &match_state)
            .await?;

        session
            .app_state
            .ephemeral_match_repo
            .push_move(&session.match_id, mv)
            .await?;

        if match_state.match_result.is_some() {
            finalize_match(session).await?;
        }

        session
            .pubsub
            .publish(
                &format!("match:{}", session.match_id),
                &PubSubMessage::PlayerMove(mv, match_state),
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
