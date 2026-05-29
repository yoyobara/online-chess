import { FC } from 'react';
import { Piece, PieceColor } from '../../../../types/piece';
import { Square as SquareType } from '../../../../types/square';

import styles from './PieceComponent.module.scss';
import { getPieceSvg } from '../../../../utils/piece';
import { useDraggable } from '@dnd-kit/core';

export interface PieceComponentProps {
  piece: Piece;
  squareName: SquareType;
  index: number;
  disabled: true | PieceColor;
}

export const PieceComponent: FC<PieceComponentProps> = ({
  piece,
  squareName,
  index,
  disabled,
}: PieceComponentProps) => {
  const [row, column] = [Math.floor(index / 8), index % 8];

  const { attributes, listeners, setNodeRef, transform } = useDraggable({
    id: `piece ${squareName}`,
    data: {
      squareName,
    },
    disabled: disabled === true || disabled === piece.piece_color,
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
      draggable={false}
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
