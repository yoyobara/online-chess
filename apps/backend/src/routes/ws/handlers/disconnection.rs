use crate::{
    internal_broadcast::InternalMessage,
    routes::ws::{message::ServerMessage, session::Session, state::SessionState},
};

pub async fn handle_client_disconnection(session: &mut Session) {
    match session.state {
        SessionState::InGame { opponent_id, .. } => {
            session
                .communicator
                .internal_notify(opponent_id, InternalMessage::OpponentDisconnected)
                .await;
        }
        SessionState::WaitingForMatch { expected_match_id } => {
            sqlx::query!("DELETE FROM matches WHERE id = $1", expected_match_id)
                .execute(&session.pool)
                .await
                .unwrap();
        }
        _ => panic!("bad state"),
    }
}

pub async fn handle_opponent_disconnection(session: &mut Session) {
    if let SessionState::InGame { white, .. } = session.state {
        sqlx::query!(
            r#"
            UPDATE matches
            SET match_status = CASE
                WHEN $1 THEN 'White_won'::match_status_enum
                ELSE 'Black_won'::match_status_enum
            END
            "#,
            white
        )
        .execute(&session.pool)
        .await
        .unwrap();

        session
            .communicator
            .ws_send(ServerMessage::OpponentDisconnected)
            .await;
    } else {
        panic!("bad state");
    }
}
