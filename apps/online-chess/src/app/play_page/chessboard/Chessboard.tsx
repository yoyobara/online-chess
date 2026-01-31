import { FC } from 'react';
import styles from './Chessboard.module.scss';
import { Square } from './square';
import { PieceComponent } from './piece';
import { Board } from '../../../types/board';
import { DndContext, DragEndEvent } from '@dnd-kit/core';

interface ChessBoardProps {
  board: Board;
}

export const Chessboard: FC<ChessBoardProps> = ({ board }) => {
  const handleDragEnd = (event: DragEndEvent) => {
    console.log(event.active.id);
    console.log(event.over?.id);
  };

  return (
    <DndContext onDragEnd={handleDragEnd}>
      <div className={styles.chessboard}>
        {Array.from({ length: 64 }).map((_, i) => (
          <Square index={i} />
        ))}
        {board.state.map((piece, i) =>
          piece ? <PieceComponent index={i} piece={piece} /> : null
        )}
      </div>
    </DndContext>
  );
};
