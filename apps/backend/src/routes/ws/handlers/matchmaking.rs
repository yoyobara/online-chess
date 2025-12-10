use crate::{
    internal_broadcast::InternalMessage,
    routes::ws::{message::ServerMessage, session::Session, state::SessionState},
};

pub async fn handle_looking_for_match(session: &mut Session) {
    assert_eq!(session.state, SessionState::Connected);

    let matchmaking_attempt = sqlx::query!(
        r#"
            WITH claimed AS (
                SELECT id
                FROM matches
                WHERE match_status = 'Matchmaking'
                AND player2_id IS NULL
                ORDER BY id
                LIMIT 1
                FOR UPDATE
            )
            UPDATE matches
            SET player2_id = $1,
                match_status = 'Ongoing'
            WHERE id IN (SELECT id FROM claimed)
            RETURNING id, player1_id;
            "#,
        session.player_id,
    )
    .fetch_optional(&session.pool)
    .await
    .unwrap();

    if let Some(matchmaking) = matchmaking_attempt {
        session
            .communicator
            .internal_notify(
                matchmaking.player1_id,
                InternalMessage::MatchFound {
                    match_id: matchmaking.id,
                    opponent_id: session.player_id,
                },
            )
            .await;

        session.state = SessionState::InGame {
            match_id: matchmaking.id,
            opponent_id: matchmaking.player1_id,
            your_turn: false,
            white: false,
        };

        session
            .communicator
            .ws_send(ServerMessage::MatchFound {
                opponent_name: matchmaking.player1_id.to_string(),
                you_are_white: false,
            })
            .await;
    } else {
        let created_match_id = sqlx::query_scalar!(
                "INSERT INTO matches (player1_id, match_status) VALUES($1, 'Matchmaking') RETURNING id;",
                session.player_id
            )
            .fetch_one(&session.pool)
            .await
            .unwrap();

        session.state = SessionState::WaitingForMatch {
            expected_match_id: created_match_id,
        };

        session
            .communicator
            .ws_send(ServerMessage::WaitingForMatch)
            .await;
    }
}

pub async fn handle_match_found(session: &mut Session, match_id: i32, opponent_id: i32) {
    assert_eq!(
        session.state,
        SessionState::WaitingForMatch {
            expected_match_id: match_id
        }
    );

    session.state = SessionState::InGame {
        match_id,
        opponent_id,
        your_turn: true,
        white: true,
    };

    session
        .communicator
        .ws_send(ServerMessage::MatchFound {
            opponent_name: opponent_id.to_string(),
            you_are_white: true,
        })
        .await;
}
