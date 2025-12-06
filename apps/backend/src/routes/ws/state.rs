#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SessionState {
    Connected,
    WaitingForMatch { expected_match_id: i32 },
    YourTurn { match_id: i32, opponent_id: i32 },
    OpponentTurn { match_id: i32, opponent_id: i32 },
}
