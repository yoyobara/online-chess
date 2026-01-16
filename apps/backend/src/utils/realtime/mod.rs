use axum::extract::ws::WebSocket;

use crate::state::AppState;

pub struct RealtimeSession {
    app_state: AppState,
    socket: WebSocket,
    player_id: i32,
    match_id: String,
}

impl RealtimeSession {
    pub fn new(app_state: AppState, socket: WebSocket, player_id: i32, match_id: String) -> Self {
        Self {
            app_state,
            socket,
            player_id,
            match_id,
        }
    }

    pub async fn mainloop(self) -> anyhow::Result<()> {
        Ok(())
    }
}
