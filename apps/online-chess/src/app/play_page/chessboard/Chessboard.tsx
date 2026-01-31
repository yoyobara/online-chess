import { FC, useMemo } from 'react';
import styles from './Chessboard.module.scss';
import { Square } from './square';
import { PieceComponent } from './piece';
import { Board } from '../../../types/board';
import { Piece } from '../../../types/piece';

interface ChessBoardProps {
  board: Board;
}

const makePiecesArray = (board: Board): (Piece | null)[] => {
  const pieces = [];

  for (let i = 7; i >= 0; i--) {
    for (let j = 0; j < 8; j++) {
      pieces.push(board.state[i * 8 + j]);
    }
  }

  return pieces;
};

export const Chessboard: FC<ChessBoardProps> = ({ board }) => {
  const pieces = useMemo(() => makePiecesArray(board), [board]);

  return (
    <div className={styles.chessboard}>
      {Array.from({ length: 64 }).map((_, i) => (
        <Square index={i} />
      ))}
      {pieces.map((piece, i) =>
        piece ? <PieceComponent index={i} piece={piece} /> : null
      )}
    </div>
  );
};
