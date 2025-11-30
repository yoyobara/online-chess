use sqlx::postgres::PgListener;

use crate::{
    routes::ws::{client_state::ClientState, messager::WsMessenger},
    state::AppState,
};

pub async fn handle_looking_for_match(
    _messenger: &mut WsMessenger,
    client_state: &mut ClientState,
    listener: &mut PgListener,
    app_state: &AppState,
    player_id: i32,
) {
    assert_eq!(*client_state, ClientState::Connected);

    let match_result = sqlx::query_scalar!(
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
        RETURNING id;
        "#,
        player_id,
    )
    .fetch_optional(&app_state.pool)
    .await
    .unwrap();

    let match_id = match match_result {
        Some(id) => {
            listener.listen(&id.to_string()).await.unwrap();

            sqlx::query!("SELECT pg_notify($1, 'matchFound');", id.to_string())
                .execute(&app_state.pool)
                .await
                .unwrap();

            id
        }

        None => {
            let new_match_id = sqlx::query_scalar!(
                "INSERT INTO matches (player1_id, match_status) VALUES($1, 'Matchmaking') RETURNING id;",
                player_id
            )
            .fetch_one(&app_state.pool)
            .await
            .unwrap();

            listener.listen(&new_match_id.to_string()).await.unwrap();

            new_match_id
        }
    };

    *client_state = ClientState::WaitingForMatch { match_id };
}
