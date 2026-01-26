use rust_chess::core::color::Color;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct MatchmakingResponse {
    pub match_id: String,
    pub side: Color,
}
