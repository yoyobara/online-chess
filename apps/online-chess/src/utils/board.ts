import { Board } from '../types/board';
import { Piece } from '../types/piece';

export const getPieceByIndex = (
  board: Board,
  squareIndex: number
): Piece | null => {
  return board.state[squareIndex];
};
