use rust_chess::{board::{Board, EndgameState}, core::{chess_move::Move, color::Color}};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum MatchResult {
    Win(Color),
    Draw,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchState {
    pub board: Board,
    pub current_turn: Color,
    pub match_result: Option<MatchResult>,
}

impl MatchState {
    pub fn is_allowed_to_move(&self, player_color: Color, mv: Move) -> bool {
        let your_turn = self.current_turn == player_color;
        let your_piece =
            self.board.get(mv.from).map(|p| p.piece_color) == Some(player_color);

        your_turn
            && your_piece
            && self
                .board
                .get_legal_moves(mv.from)
                .unwrap_or_default()
                .contains(&mv)
    }

    pub fn apply_move(&mut self, mv: Move) {
        self.board.apply_move(mv);
        self.current_turn = !self.current_turn;
        self.update_match_result();
    }

    fn update_match_result(&mut self) {
        let opponent_color = !self.current_turn;
        let endgame_state = self
            .board
            .is_player_under_endgame_state(self.current_turn);

        self.match_result = endgame_state.map(|state| match state {
            EndgameState::Checkmate => MatchResult::Win(opponent_color),
            EndgameState::Stalemate => MatchResult::Draw,
        });
    }
}
