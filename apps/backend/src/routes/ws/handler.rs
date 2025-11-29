use sqlx::postgres::PgListener;

use crate::{
    routes::ws::{message::ClientMessage, messager::WsMessager},
    state::AppState,
};

pub async fn handle_socket(mut messager: WsMessager, player_id: i32, state: AppState) {
    let join_msg = messager.recv().await;
    assert_eq!(join_msg, ClientMessage::LookingForMatch);

    let available_player = sqlx::query_scalar!(
        "SELECT waiting_player_id FROM waiting_room JOIN users ON waiting_player_id = id WHERE ABS((SELECT rank FROM users WHERE id = $1) - rank) < 30", player_id
    ).fetch_optional(&state.pool).await.unwrap();

    unimplemented!()
}
