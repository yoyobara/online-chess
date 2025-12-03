mod client;
mod server;

pub use client::ClientMessage;
pub use server::ServerMessage;

use crate::internal_broadcast::InternalMessage;

#[derive(Debug)]
pub enum Event {
    ClientMessage(ClientMessage),
    InternalMessage(InternalMessage),
}
