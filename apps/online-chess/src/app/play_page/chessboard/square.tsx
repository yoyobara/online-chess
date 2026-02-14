import { FC } from 'react';
import styles from './Chessboard.module.scss';
import { useDroppable } from '@dnd-kit/core';

export interface SquareProps {
  index: number;
}

export const Square: FC<SquareProps> = ({ index }: SquareProps) => {
  const [row, column] = [Math.floor(index / 8), index % 8];

  const { setNodeRef } = useDroppable({
    id: `square ${index}`,
    data: {
      index,
    },
  });

  return (
    <div
      ref={setNodeRef}
      className={`${styles.square} ${
        (row + column) % 2 ? styles.white : styles.black
      }`}
      style={{
        width: '12.5%',
        height: '12.5%',
        top: `calc(12.5% * ${7 - row})`,
        left: `calc(12.5% * ${column})`,
      }}
    />
  );
};
