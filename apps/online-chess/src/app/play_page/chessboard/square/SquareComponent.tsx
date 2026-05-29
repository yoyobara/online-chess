import { FC } from 'react';
import styles from './Square.module.scss';
import { useDroppable } from '@dnd-kit/core';
import { Square as SquareType } from '../../../../types/square';
import { getSquareColor, getSquareIndex } from '../../../../utils/square';

export interface SquareComponentProps {
  squareName: SquareType;
  index: number;
}

export const SquareComponent: FC<SquareComponentProps> = ({
  squareName,
  index,
}: SquareComponentProps) => {
  const color = getSquareColor(getSquareIndex(squareName));
  const [row, column] = [Math.floor(index / 8), index % 8];

  const { setNodeRef } = useDroppable({
    id: `square ${squareName}`,
    data: {
      name: squareName,
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
