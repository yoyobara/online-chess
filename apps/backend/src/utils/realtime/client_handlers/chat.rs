use crate::{
    models::chat::ChatMessage,
    utils::{pubsub::message::PubSubMessage, realtime::RealtimeSession},
};

pub async fn handle_client_chat(
    session: &mut RealtimeSession,
    content: String,
) -> anyhow::Result<()> {
    let chat_msg = ChatMessage {
        id: uuid::Uuid::new_v4().to_string(),
        author_id: session.player_id,
        content,
    };

    session
        .pubsub
        .publish(
            &format!("match:{}", session.match_id),
            &PubSubMessage::ChatMessage(chat_msg),
        )
        .await?;

    Ok(())
}
