import { FC } from 'react';
import styles from './Chessboard.module.scss';
import { useDroppable } from '@dnd-kit/core';

export interface SquareProps {
  index: number;
}

export const Square: FC<SquareProps> = ({ index }: SquareProps) => {
  //   const { isOver, setNodeRef } = useDroppable({
  //     id: `square ${index}`,
  //   });

  const [row, column] = [Math.floor(index / 8), index % 8];

  return (
    <div
      className={`${styles.square} ${
        (row + column) % 2 ? styles.black : styles.white
      }`}
      style={{
        width: '12.5%',
        height: '12.5%',
        top: `calc(12.5% * ${row})`,
        left: `calc(12.5% * ${column})`,
      }}
    />
  );
};
