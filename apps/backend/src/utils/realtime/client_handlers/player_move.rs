use anyhow::Result;
use rust_chess::core::chess_move::Move;

use crate::{
    utils::{
        pubsub::message::PubSubMessage,
        realtime::{client_communication::message::ServerMessage, RealtimeSession},
    },
};

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

    if match_state.is_allowed_to_move(session.player_color, mv) {
        match_state.apply_move(mv);

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
