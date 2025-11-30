#[derive(PartialEq, Eq, Debug)]
pub enum ClientState {
    Connected,
    WaitingForMatch { match_id: i32 },
    InMatch,
}
