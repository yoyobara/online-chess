import _ from 'lodash';
import { Board } from '../types/board';
import { Move, CastlingType, MoveType } from '../types/move';
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
  const piece = board.state[srcIndex];

  if (piece?.piece_type === 'Pawn') {
    const srcFile = srcIndex % 8;
    const destFile = destIndex % 8;

    if (srcFile !== destFile && board.state[destIndex] === null) {
      return 'EnPassant';
    }
  }

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

export const applyMove = (
  board: Board,
  move: Move,
  myColor: PieceColor
): Board => {
  const newBoard = _.cloneDeep(board);
  const { srcIndex, destIndex, promotion, moveType } = move;

  newBoard.state[destIndex] = newBoard.state[srcIndex];
  newBoard.state[srcIndex] = null;

  if (promotion) {
    newBoard.state[destIndex]!.piece_type = promotion;
  }

  if (moveType === 'KingsideCastling' || moveType === 'QueensideCastling') {
    const castlingMove = getCastlingMove(myColor, moveType);

    newBoard.state[castlingMove.rookDest] =
      newBoard.state[castlingMove.rookSrc];
    newBoard.state[castlingMove.rookSrc] = null;
  }

  if (moveType === 'EnPassant') {
    const destFile = destIndex % 8;
    const srcRank = Math.floor(srcIndex / 8);
    const capturedPawnIndex = srcRank * 8 + destFile;
    newBoard.state[capturedPawnIndex] = null;
  }

  return newBoard;
};
