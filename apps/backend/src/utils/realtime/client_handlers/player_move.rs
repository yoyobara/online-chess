use anyhow::Result;
use rust_chess::core::chess_move::Move as ChessMove;

use crate::utils::{
    pubsub::message::PubSubMessage,
    realtime::{
        client_communication::message::{PlayerMoveData, ServerMessage},
        RealtimeSession,
    },
};

pub async fn handle_client_player_move(
    session: &mut RealtimeSession,
    move_data: PlayerMoveData,
) -> Result<()> {
    let mut match_state = session
        .app_state
        .match_repo
        .get_match_state(&session.match_id)
        .await?;

    match_state.board.apply_move(ChessMove::new(
        move_data.src_square,
        move_data.dest_square,
        None,
        None,
    ));
    match_state.move_count += 1;

    session
        .communicator
        .send(ServerMessage::MoveResult(true))
        .await?;

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

    Ok(())
}
