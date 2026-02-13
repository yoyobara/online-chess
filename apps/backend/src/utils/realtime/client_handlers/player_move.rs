use anyhow::Result;
use rust_chess::core::chess_move::Move;
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
    let mv = ChessMove::new(move_data.src_square, move_data.dest_square, None, None);

    let mut match_state = session
        .app_state
        .match_repo
        .get_match_state(&session.match_id)
        .await?;

    let moves = match_state.board.get_legal_moves(mv.from).unwrap();

    if moves.contains(&mv) {
        match_state.board.apply_move(mv);
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
    } else {
        session
            .communicator
            .send(ServerMessage::MoveResult(false))
            .await?;
    }
    Ok(())
}
