import { FC } from 'react';
import { Piece } from '../../../types/piece';

import styles from './Chessboard.module.scss';
import { getPieceSvg } from '../../../utils/piece';
import { useDraggable } from '@dnd-kit/core';

export interface PieceComponentProps {
  piece: Piece;
  squareNumber: number;
  index: number;
}

export const PieceComponent: FC<PieceComponentProps> = ({
  piece,
  squareNumber,
  index,
}: PieceComponentProps) => {
  const [row, column] = [Math.floor(index / 8), index % 8];

  const { attributes, listeners, setNodeRef, transform } = useDraggable({
    id: `piece ${squareNumber}`,
    data: {
      squareNumber,
    },
  });

  const style = transform
    ? {
        transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
      }
    : undefined;

  return (
    <img
      ref={setNodeRef}
      className={styles.piece}
      src={getPieceSvg(piece)}
      alt={`${piece.piece_color} ${piece.piece_type}`}
      style={{
        width: '12.5%',
        height: '12.5%',
        top: `calc(12.5% * ${7 - row})`,
        left: `calc(12.5% * ${column})`,
        ...style,
      }}
      {...listeners}
      {...attributes}
    />
  );
};
