import { PieceType } from './piece';

export type MoveType =
  | 'Quiet'
  | 'Capture'
  | 'QueensideCastling'
  | 'KingsideCastling';

export type Move = {
  srcIndex: number;
  destIndex: number;
  promotion: PieceType | null;
  moveType: MoveType;
};
