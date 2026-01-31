import { FC } from 'react';
import { Piece } from '../../../types/piece';

import styles from './Chessboard.module.scss';
import { getPieceSvg } from '../../../utils/piece';

export interface PieceComponentProps {
  piece: Piece;
  index: number;
}

export const PieceComponent: FC<PieceComponentProps> = ({
  piece,
  index,
}: PieceComponentProps) => {
  const [row, column] = [Math.floor(index / 8), index % 8];

  return (
    <img
      className={styles.piece}
      src={getPieceSvg(piece)}
      alt={`${piece.piece_color} ${piece.piece_type}`}
      style={{
        width: '12.5%',
        height: '12.5%',
        top: `calc(12.5% * ${row})`,
        left: `calc(12.5% * ${column})`,
      }}
    />
  );
};
