import { FC } from 'react';
import styles from './Chessboard.module.scss';
import { Square } from './square';
import { PieceComponent } from './piece';
import { Board } from '../../../types/board';
import { DndContext, DragEndEvent } from '@dnd-kit/core';
import { PieceColor } from '../../../types/piece';

interface ChessBoardProps {
  board: Board;
  handleMove: (srcIndex: number, destIndex: number) => void;
  myColor: PieceColor;
}

export const Chessboard: FC<ChessBoardProps> = ({
  board,
  handleMove,
  myColor,
}) => {
  const onDragEnd = (ev: DragEndEvent) => {
    if (!ev.over) return;

    const destSquareNumber: number = ev.over.data.current?.squareNumber;
    const srcSquareNumber: number = ev.active.data.current?.squareNumber;

    handleMove(srcSquareNumber, destSquareNumber);
  };

  return (
    <DndContext onDragEnd={onDragEnd}>
      <div className={styles.chessboard}>
        {board.state.map((piece, i) => (
          <>
            <Square squareNumber={i} index={myColor === 'White' ? i : 63 - i} />
            {piece ? (
              <PieceComponent
                squareNumber={i}
                index={myColor === 'White' ? i : 63 - i}
                piece={piece}
              />
            ) : null}
          </>
        ))}
      </div>
    </DndContext>
  );
};
