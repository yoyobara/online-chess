import { FC } from 'react';
import boardSvg from '../../../assets/board.svg';
import styles from './Chessboard.module.scss';
import { Piece } from '../../../types/piece';
import { getPieceSvg } from '../../../utils/piece';

interface chessBoardProps {
  pieces: Piece[];
}

export const pieceElement = (piece: Piece, index: number) => {
  const [row, column] = [Math.floor(index / 8), index % 8];

  return (
    <img
      className={styles.piece}
      src={getPieceSvg(piece)}
      alt={`${piece.color} ${piece.type}`}
      style={{
        width: '12.5%',
        height: '12.5%',
        top: `calc(12.5% * ${row})`,
        left: `calc(12.5% * ${column})`,
      }}
    />
  );
};

export const Chessboard: FC<chessBoardProps> = ({ pieces }) => {
  return (
    <div className={styles.chessboard}>
      <img className={styles.board_img} src={boardSvg} alt="chessboard" />
      {pieces.map(pieceElement)}
    </div>
  );
};
