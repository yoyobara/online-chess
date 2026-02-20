import { FC } from 'react';
import styles from './Chessboard.module.scss';
import { Square } from './square';
import { PieceComponent } from './piece';
import { Board } from '../../../types/board';
import { DndContext, DragEndEvent } from '@dnd-kit/core';
import { PieceColor } from '../../../types/piece';
import { getSquareName } from '../../../utils/square';
import { useRealtime } from '../../../contexts/realtime';
import { getPieceByIndex } from '../../../utils/board';

interface ChessBoardProps {
  board: Board;
  myColor: PieceColor;
  disableDrag: true | PieceColor;
}

export const Chessboard: FC<ChessBoardProps> = ({
  board,
  myColor,
  disableDrag,
}) => {
  const { sendMessage } = useRealtime();

  const handleMove = (srcIndex: number, destIndex: number) => {
    if (srcIndex === destIndex) {
      return;
    }

    const src = getSquareName(srcIndex);
    const dest = getSquareName(destIndex);

    console.log(`${src} to ${dest}`);
    sendMessage({
      type: 'PlayerMove',
      data: {
        src_square: src,
        dest_square: dest,
        captured_piece: getPieceByIndex(board, destIndex)?.piece_type ?? null,
      },
    });
  };

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
                disabled={disableDrag}
              />
            ) : null}
          </>
        ))}
      </div>
    </DndContext>
  );
};
