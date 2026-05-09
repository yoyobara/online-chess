import { PieceType } from './piece';

export type CastlingType = 'QueensideCastling' | 'KingsideCastling';

export type MoveType = 'Quiet' | 'Capture' | 'EnPassant' | CastlingType;

export type Move = {
  srcIndex: number;
  destIndex: number;
  promotion: PieceType | null;
  moveType: MoveType;
};
