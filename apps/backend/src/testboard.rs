use rust_chess::board::Board;
use rust_chess::core::color::Color;
use rust_chess::core::piece::{Piece, PieceType};
use rust_chess::core::square::Square;

pub fn get_test_board() -> Board {
    let mut b = Board::empty();

    b.set(Square::E1, Some(Piece::new(PieceType::King, Color::White)));
    b.set(Square::E8, Some(Piece::new(PieceType::King, Color::Black)));
    b.set(Square::E2, Some(Piece::new(PieceType::Pawn, Color::White)));
    b.set(Square::F4, Some(Piece::new(PieceType::Pawn, Color::Black)));

    b
}
