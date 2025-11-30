use sqlx::postgres::PgListener;

use crate::{
    routes::ws::{message::ClientMessage, messager::WsMessager},
    state::AppState,
};

pub async fn handle_socket(mut messager: WsMessager, player_id: i32, state: AppState) {
    // get a looingForMatch message
    assert_eq!(messager.recv().await, ClientMessage::LookingForMatch);
}
