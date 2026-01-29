export type PieceType =
  | 'Pawn'
  | 'Rook'
  | 'Knight'
  | 'Bishop'
  | 'Queen'
  | 'King';
export type PieceColor = 'Black' | 'White';

export interface Piece {
  piece_type: PieceType;
  piece_color: PieceColor;
}
