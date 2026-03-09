import { PieceType } from './piece';

export type Move = {
  srcIndex: number;
  destIndex: number;
  capturedPiece: PieceType | null;
};
