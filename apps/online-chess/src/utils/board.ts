import { Board } from '../types/board';
import { CastlingType, MoveType } from '../types/move';
import { Piece, PieceColor } from '../types/piece';

export const getPieceByIndex = (
  board: Board,
  squareIndex: number
): Piece | null => {
  return board.state[squareIndex];
};

export const isOnPromotionRow = (squareIndex: number, myColor: PieceColor) => {
  if (myColor === 'White') {
    return squareIndex > 55;
  } else {
    return squareIndex < 8;
  }
};

export const determineMoveType = (
  board: Board,
  srcIndex: number,
  destIndex: number
): MoveType => {
  if (srcIndex === 4 && destIndex === 6) {
    return 'KingsideCastling';
  } else if (srcIndex === 4 && destIndex === 2) {
    return 'QueensideCastling';
  } else if (srcIndex === 60 && destIndex === 62) {
    return 'KingsideCastling';
  } else if (srcIndex === 60 && destIndex === 58) {
    return 'QueensideCastling';
  } else if (board.state[destIndex] === null) {
    return 'Quiet';
  } else {
    return 'Capture';
  }
};

const CASTLING_MOVES: Record<
  `${PieceColor}-${CastlingType}`,
  { rookSrc: number; rookDest: number }
> = {
  'White-KingsideCastling': {
    rookSrc: 7,
    rookDest: 5,
  },
  'White-QueensideCastling': {
    rookSrc: 0,
    rookDest: 3,
  },
  'Black-KingsideCastling': {
    rookSrc: 63,
    rookDest: 61,
  },
  'Black-QueensideCastling': {
    rookSrc: 56,
    rookDest: 59,
  },
};

export const getCastlingMove = (color: PieceColor, type: CastlingType) => {
  return CASTLING_MOVES[`${color}-${type}`];
};
