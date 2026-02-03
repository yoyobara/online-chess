import { FC } from 'react';
import styles from './Chessboard.module.scss';
import { Square } from './square';
import { PieceComponent } from './piece';
import { Board } from '../../../types/board';
import { DndContext, DragEndEvent } from '@dnd-kit/core';

interface ChessBoardProps {
  board: Board;
  handleMove: (src: string, dest: string) => void;
}

export const Chessboard: FC<ChessBoardProps> = ({ board, handleMove }) => {
  const onDragEnd = (ev: DragEndEvent) => {
    if (!ev.over) return;

    const destSquare: string = ev.over.data.current?.squareName;
    const srcSquare: string = ev.active.data.current?.squareName;

    handleMove(srcSquare, destSquare);
  };

  return (
    <DndContext onDragEnd={onDragEnd}>
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
