import _ from 'lodash';
import { Board, CastlingRights } from '../types/board';
import { Move } from '../types/move';
import { Piece, PieceColor } from '../types/piece';
import { isPawn } from './piece';
import { getRank, getFile, fromCoords } from './square';
import { getCastlingMove } from './move';

export const getPieceByIndex = (
  board: Board,
  squareIndex: number
): Piece | null => {
  return board.state[squareIndex];
};

export const isOnPromotionRow = (squareIndex: number, myColor: PieceColor) => {
  const rank = getRank(squareIndex);
  return myColor === 'White' ? rank === 7 : rank === 0;
};

const updateCastlingRights = (
  rights: [CastlingRights, CastlingRights],
  srcIndex: number,
  destIndex: number
): [CastlingRights, CastlingRights] => {
  const newRights = _.cloneDeep(rights);

  // If king or rook moves, lose rights
  [srcIndex, destIndex].forEach((idx) => {
    if (idx === 4) newRights[0] = { kingside: false, queenside: false };
    if (idx === 60) newRights[1] = { kingside: false, queenside: false };
    if (idx === 0) newRights[0].queenside = false;
    if (idx === 7) newRights[0].kingside = false;
    if (idx === 56) newRights[1].queenside = false;
    if (idx === 63) newRights[1].kingside = false;
  });

  return newRights;
};

export const applyMove = (
  board: Board,
  move: Move,
  myColor: PieceColor
): Board => {
  const newBoard = _.cloneDeep(board);
  const { srcIndex, destIndex, promotion, moveType } = move;
  const piece = newBoard.state[srcIndex];

  if (!piece) return board;

  // 1. Move the piece
  newBoard.state[destIndex] = piece;
  newBoard.state[srcIndex] = null;

  // 2. Handle Promotion
  if (promotion) {
    newBoard.state[destIndex]!.piece_type = promotion;
  }

  // 3. Handle Special Moves
  if (moveType === 'KingsideCastling' || moveType === 'QueensideCastling') {
    const castlingMove = getCastlingMove(myColor, moveType);
    newBoard.state[castlingMove.rookDest] = newBoard.state[castlingMove.rookSrc];
    newBoard.state[castlingMove.rookSrc] = null;
  }

  if (moveType === 'EnPassant') {
    const capturedPawnIndex = fromCoords(getRank(srcIndex), getFile(destIndex));
    newBoard.state[capturedPawnIndex] = null;
  }

  // 4. Update En Passant Square
  newBoard.en_passant_square = null;
  if (isPawn(piece)) {
    const srcRank = getRank(srcIndex);
    const destRank = getRank(destIndex);
    if (Math.abs(srcRank - destRank) === 2) {
      newBoard.en_passant_square = fromCoords((srcRank + destRank) / 2, getFile(srcIndex));
    }
  }

  // 5. Update Castling Rights
  newBoard.castling_rights = updateCastlingRights(newBoard.castling_rights, srcIndex, destIndex);

  return newBoard;
};
