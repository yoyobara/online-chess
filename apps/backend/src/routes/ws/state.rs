#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SessionState {
    Connected,
    WaitingForMatch {
        expected_match_id: i32,
    },
    InGame {
        match_id: i32,
        opponent_id: i32,
        white: bool,
        your_turn: bool,
    },
}
