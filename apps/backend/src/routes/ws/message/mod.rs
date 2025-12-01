mod client;
mod internal;
mod server;

pub use client::ClientMessage;
pub use internal::{InternalMessage, InternalMessageWithReciever};
pub use server::ServerMessage;

#[derive(Debug)]
pub enum Event {
    ClientMessage(ClientMessage),
    InternalMessage(InternalMessage),
}
