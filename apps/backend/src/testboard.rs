use rust_chess::board::Board;

pub fn get_test_board() -> Board {
    // let mut b = Board::empty();

    // b.set(Square::F8, Some(Piece::new(PieceType::King, Color::White)));
    // b.set(Square::H8, Some(Piece::new(PieceType::King, Color::Black)));
    // b.set(Square::C1, Some(Piece::new(PieceType::Rook, Color::White)));

    Board::default()
}
