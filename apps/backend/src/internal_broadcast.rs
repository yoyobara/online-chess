use sqlx::{postgres::PgListener, Pool, Postgres};
use tokio::{
    sync::broadcast::{self, Sender},
    task::JoinHandle,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum InternalMessage {
    MatchFound { match_id: i32, opponent_id: i32 },
    OpponentDisconnected,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct InternalMessageWithMetadata {
    pub from_user: i32,
    pub to_user: i32,
    pub message: InternalMessage,
}

pub async fn start_internal_broadcast(
    pool: Pool<Postgres>,
) -> (
    Sender<InternalMessageWithMetadata>,
    JoinHandle<anyhow::Result<()>>,
) {
    let (tx, _rx) = broadcast::channel::<InternalMessageWithMetadata>(1024);
    let handler = tokio::spawn(internal_broadcast(pool, tx.clone()));

    (tx, handler)
}

async fn internal_broadcast(
    pool: Pool<Postgres>,
    tx: Sender<InternalMessageWithMetadata>,
) -> anyhow::Result<()> {
    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen("game_events").await?;

    while let Ok(msg) = listener.recv().await {
        let deserialized: InternalMessageWithMetadata = serde_json::from_str(msg.payload())?;

        tx.send(deserialized)?;
    }

    Ok(())
}
