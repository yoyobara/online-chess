use rust_chess::board::Board;
use rust_chess::core::color::Color;
use rust_chess::core::piece::{Piece, PieceType};
use rust_chess::core::square::Square;

pub fn get_test_board() -> Board {
    let mut b = Board::empty();

    b.set(Square::E1, Some(Piece::new(PieceType::King, Color::White)));
    b.set(Square::E8, Some(Piece::new(PieceType::King, Color::Black)));
    b.set(Square::H1, Some(Piece::new(PieceType::Rook, Color::White)));
    b.set(Square::A8, Some(Piece::new(PieceType::Rook, Color::Black)));

    b
}
