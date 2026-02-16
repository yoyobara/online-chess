import { FC } from 'react';
import styles from './Chessboard.module.scss';
import { useDroppable } from '@dnd-kit/core';
import { getSquareColor, getSquareName } from '../../../utils/square';

export interface SquareProps {
  squareNumber: number;
  index: number;
}

export const Square: FC<SquareProps> = ({
  squareNumber,
  index,
}: SquareProps) => {
  const name = getSquareName(squareNumber);
  const color = getSquareColor(squareNumber);
  const [row, column] = [Math.floor(index / 8), index % 8];

  const { setNodeRef } = useDroppable({
    id: `square ${name}`,
    data: {
      name,
      squareNumber,
      color,
    },
  });

  return (
    <div
      ref={setNodeRef}
      className={`${styles.square} ${
        color === 'Light' ? styles.light : styles.dark
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
