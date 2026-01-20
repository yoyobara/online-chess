use serde::Serialize;

#[derive(Serialize)]
pub enum ServerMessage {
    Bar(i32),
}
