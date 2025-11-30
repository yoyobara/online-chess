use sqlx::postgres::PgListener;

use crate::{
    routes::ws::{message::ClientMessage, messager::WsMessager},
    state::AppState,
};

pub async fn handle_socket(mut messager: WsMessager, player_id: i32, state: AppState) {
    // get a looingForMatch message
    assert_eq!(messager.recv().await, ClientMessage::LookingForMatch);

    // look for match
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
    .fetch_optional(&state.pool)
    .await
    .unwrap();

    messager
        .socket
        .send(axum::extract::ws::Message::text(format!(
            "{:?}",
            match_result
        )))
        .await
        .unwrap();

    // regardless of result, start listening
    let mut listener = PgListener::connect_with(&state.pool).await.unwrap();

    if let Some(match_id) = match_result {
        listener.listen(&match_id.to_string()).await.unwrap();

        sqlx::query!("SELECT pg_notify($1, 'matchFound');", match_id.to_string())
            .execute(&state.pool)
            .await
            .unwrap();
    } else {
        let new_match_id = sqlx::query_scalar!(
            "INSERT INTO matches (player1_id, match_status) VALUES($1, 'Matchmaking') RETURNING id;",
            player_id
        )
        .fetch_one(&state.pool)
        .await
        .unwrap();

        listener.listen(&new_match_id.to_string()).await.unwrap();
    }

    // now the listener should get a match found message.
    assert_eq!(listener.recv().await.unwrap().payload(), "matchFound");
    messager
        .send(crate::routes::ws::message::ServerMessage::MatchFound)
        .await;
}
