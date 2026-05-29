import { PieceType } from './piece';
import { Square } from './square';

export type CastlingType = 'QueensideCastling' | 'KingsideCastling';

export type MoveType = 'Quiet' | 'Capture' | 'EnPassant' | CastlingType;

export type Move = {
  src_square: Square;
  dest_square: Square;
  promotion: PieceType | null;
  move_type: MoveType;
};
