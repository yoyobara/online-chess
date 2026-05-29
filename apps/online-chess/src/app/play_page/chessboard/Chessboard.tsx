import React, { FC } from 'react';
import styles from './Chessboard.module.scss';
import { SquareComponent } from './square/SquareComponent';
import { PieceComponent } from './piece_component/PieceComponent';
import { Board } from '../../../types/board';
import { DndContext, DragEndEvent } from '@dnd-kit/core';
import { PieceColor } from '../../../types/piece';
import { Square } from '../../../types/square';
import { getSquareIndex, getSquareName } from '../../../utils/square';
import { useRealtime } from '../../../contexts/realtime';
import { Move } from '../../../types/move';
import { determineMoveType, isOnPromotionRow } from '../../../utils/board';

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
        from: getSquareName(srcIndex),
        to: getSquareName(destIndex),
        move_type: moveType,
        promotion: null,
      });
    } else {
      const move: Move = {
        from: getSquareName(srcIndex),
        to: getSquareName(destIndex),
        promotion: null,
        move_type: moveType,
      };
      sendMessage({
        type: 'PlayerMove',
        data: move,
      });
      setWaitingForMoveResponse(move);
    }
  };

  const onDragEnd = (ev: DragEndEvent) => {
    if (!ev.over) return;

    const destSquareName: Square = ev.over.data.current?.name;
    const srcSquareName: Square = ev.active.data.current?.squareName;

    if (destSquareName && srcSquareName) {
      handleMove(getSquareIndex(srcSquareName), getSquareIndex(destSquareName));
    }
  };

  return (
    <DndContext onDragEnd={onDragEnd}>
      <div className={styles.chessboard}>
        {board.state.map((piece, i) => (
          <React.Fragment key={i}>
            <SquareComponent
              squareName={getSquareName(i)}
              index={myColor === 'White' ? i : 63 - i}
            />
            {piece ? (
              <PieceComponent
                squareName={getSquareName(i)}
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
