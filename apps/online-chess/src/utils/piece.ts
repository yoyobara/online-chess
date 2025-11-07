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
  const { type, color } = piece;

  switch (type) {
    case 'king':
      return color === 'white' ? kingW : kingB;
    case 'queen':
      return color === 'white' ? queenW : queenB;
    case 'rook':
      return color === 'white' ? rookW : rookB;
    case 'bishop':
      return color === 'white' ? bishopW : bishopB;
    case 'knight':
      return color === 'white' ? knightW : knightB;
    case 'pawn':
      return color === 'white' ? pawnW : pawnB;
  }
};
