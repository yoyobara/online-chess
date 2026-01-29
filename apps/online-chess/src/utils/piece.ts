import kingW from '../assets/pieces/king-w.svg';
import kingB from '../assets/pieces/king-b.svg';
import queenW from '../assets/pieces/queen-w.svg';
import queenB from '../assets/pieces/queen-b.svg';
import rookW from '../assets/pieces/rook-w.svg';
import rookB from '../assets/pieces/rook-b.svg';
import bishopW from '../assets/pieces/bishop-w.svg';
import bishopB from '../assets/pieces/bishop-b.svg';
import knightW from '../assets/pieces/knight-w.svg';
import knightB from '../assets/pieces/knight-b.svg';
import pawnW from '../assets/pieces/pawn-w.svg';
import pawnB from '../assets/pieces/pawn-b.svg';

import { Piece } from '../types/piece';

export const getPieceSvg = (piece: Piece): string => {
  const { piece_type, piece_color } = piece;

  switch (piece_type) {
    case 'King':
      return piece_color === 'White' ? kingW : kingB;
    case 'Queen':
      return piece_color === 'White' ? queenW : queenB;
    case 'Rook':
      return piece_color === 'White' ? rookW : rookB;
    case 'Bishop':
      return piece_color === 'White' ? bishopW : bishopB;
    case 'Knight':
      return piece_color === 'White' ? knightW : knightB;
    case 'Pawn':
      return piece_color === 'White' ? pawnW : pawnB;
  }
};
