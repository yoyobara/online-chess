use sqlx::{postgres::PgListener, Pool, Postgres};
use tokio::sync::broadcast;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum InternalMessage {
    MatchFound { match_id: i32, opponent_id: i32 },
    OpponentDisconnected,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct InternalMessageWithReciever {
    pub to_user: i32,
    pub message: InternalMessage,
}

pub async fn start_internal_broadcast(
    pool: Pool<Postgres>,
) -> (
    tokio::sync::broadcast::Sender<InternalMessageWithReciever>,
    tokio::task::JoinHandle<()>,
) {
    let (tx, _rx) = broadcast::channel::<InternalMessageWithReciever>(1024);

    let tx1 = tx.clone();
    let handler = tokio::spawn(async move {
        let mut listener = PgListener::connect_with(&pool).await.unwrap();
        listener.listen("game_events").await.unwrap();

        while let Ok(msg) = listener.recv().await {
            let deserialized: InternalMessageWithReciever =
                serde_json::from_str(msg.payload()).unwrap();

            tx1.send(deserialized).unwrap();
        }
    });

    (tx, handler)
}
