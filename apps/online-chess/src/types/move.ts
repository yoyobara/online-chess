import { PieceType } from './piece';
import { Square } from './square';

export type CastlingType = 'QueensideCastling' | 'KingsideCastling';

export type MoveType = 'Quiet' | 'Capture' | 'EnPassant' | CastlingType;

export type Move = {
  from: Square;
  to: Square;
  promotion: PieceType | null;
  move_type: MoveType;
};
