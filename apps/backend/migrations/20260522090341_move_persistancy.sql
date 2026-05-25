-- Add migration script here
CREATE TYPE move_type_enum AS ENUM ('quiet', 'capture', 'en_passant', 'queenside_castling', 'kingside_castling');
CREATE TYPE piece_type_enum AS ENUM ('pawn', 'rook', 'knight', 'bishop', 'king', 'queen');

CREATE TABLE moves (
    id SERIAL PRIMARY KEY,
    match_id INTEGER NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    move_order INTEGER NOT NULL,
    from_square INTEGER NOT NULL CHECK (from_square >= 0 AND from_square <= 63),
    to_square INTEGER NOT NULL CHECK (to_square >= 0 AND to_square <= 63),
    move_type move_type_enum NOT NULL,
    promotion piece_type_enum,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    UNIQUE (match_id, move_order)
);

CREATE INDEX idx_moves_match_id ON moves(match_id);

CREATE TRIGGER moves_set_updated_at
BEFORE UPDATE ON moves
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();
