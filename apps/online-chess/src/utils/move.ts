import { Board } from '../types/board';
import { CastlingType, MoveType } from '../types/move';
import { PieceColor } from '../types/piece';
import { isPawn, isKing } from './piece';
import { getFile } from './square';

export const determineMoveType = (
  board: Board,
  srcIndex: number,
  destIndex: number
): MoveType => {
  const piece = board.state[srcIndex];

  if (isPawn(piece)) {
    const srcFile = getFile(srcIndex);
    const destFile = getFile(destIndex);

    if (srcFile !== destFile && board.state[destIndex] === null) {
      return 'EnPassant';
    }
  }

  if (isKing(piece)) {
    if (srcIndex === 4 && destIndex === 6) return 'KingsideCastling';
    if (srcIndex === 4 && destIndex === 2) return 'QueensideCastling';
    if (srcIndex === 60 && destIndex === 62) return 'KingsideCastling';
    if (srcIndex === 60 && destIndex === 58) return 'QueensideCastling';
  }

  if (board.state[destIndex] === null) {
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
