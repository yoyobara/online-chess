use serde::Serialize;

#[derive(Clone, PartialEq, Eq, Serialize, Debug)]
#[serde(tag = "type")]
pub enum ServerMessage {
    Log {
        message: String,
    },
    WaitingForMatch,
    MatchFound {
        opponent_name: String,
        you_are_white: bool,
    },
}
