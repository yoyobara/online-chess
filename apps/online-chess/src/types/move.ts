import { PieceType } from './piece';

export type CastlingType = 'QueensideCastling' | 'KingsideCastling';

export type MoveType = 'Quiet' | 'Capture' | 'EnPassant' | CastlingType;

export type Move = {
  srcIndex: number;
  destIndex: number;
  promotion: PieceType | null;
  moveType: MoveType;
};

export type MoveDTO = {
  src_square: string;
  dest_square: string;
  promotion: PieceType | null;
  move_type: MoveType;
};
