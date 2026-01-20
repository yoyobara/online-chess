use serde::Deserialize;

#[derive(Deserialize)]
pub enum ClientMessage {
    Foo(i32),
}
