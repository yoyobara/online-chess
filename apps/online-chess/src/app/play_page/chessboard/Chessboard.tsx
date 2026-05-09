import React, { FC } from 'react';
import styles from './Chessboard.module.scss';
import { Square } from './square/Square';
import { PieceComponent } from './piece_component/PieceComponent';
import { Board } from '../../../types/board';
import { DndContext, DragEndEvent } from '@dnd-kit/core';
import { PieceColor } from '../../../types/piece';
import { getSquareName } from '../../../utils/square';
import { useRealtime } from '../../../contexts/realtime';
import { Move } from '../../../types/move';
import {
  isOnPromotionRow,
} from '../../../utils/board';
import { determineMoveType } from '../../../utils/move';

interface ChessBoardProps {
  board: Board;
  myColor: PieceColor;
  disableDrag: true | PieceColor;
  setWaitingForMoveResponse: (optimisticMove: Move) => void;
  setWaitingForPromotionChoice: (move: Move) => void;
}

export const Chessboard: FC<ChessBoardProps> = ({
  board,
  myColor,
  disableDrag,
  setWaitingForMoveResponse,
  setWaitingForPromotionChoice,
}) => {
  const { sendMessage } = useRealtime();

  const handleMove = (srcIndex: number, destIndex: number) => {
    if (srcIndex === destIndex) {
      return;
    }

    const movedPiece = board.state[srcIndex]?.piece_type;
    const moveType = determineMoveType(board, srcIndex, destIndex);

    if (movedPiece === 'Pawn' && isOnPromotionRow(destIndex, myColor)) {
      setWaitingForPromotionChoice({
        srcIndex,
        destIndex,
        moveType,
        promotion: null,
      });
    } else {
      sendMessage({
        type: 'PlayerMove',
        data: {
          src_square: getSquareName(srcIndex),
          dest_square: getSquareName(destIndex),
          promotion: null,
          move_type: moveType,
        },
      });
      setWaitingForMoveResponse({
        srcIndex,
        destIndex,
        moveType,
        promotion: null,
      });
    }
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
          <React.Fragment key={i}>
            <Square squareNumber={i} index={myColor === 'White' ? i : 63 - i} />
            {piece ? (
              <PieceComponent
                squareNumber={i}
                index={myColor === 'White' ? i : 63 - i}
                piece={piece}
                disabled={disableDrag}
              />
            ) : null}
          </React.Fragment>
        ))}
      </div>
    </DndContext>
  );
};
