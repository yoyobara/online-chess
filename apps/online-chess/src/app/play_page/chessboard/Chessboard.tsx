import { FC, useMemo } from 'react';
import styles from './Chessboard.module.scss';
import { Square } from './square/Square';
import { PieceComponent } from './piece_component/PieceComponent';
import { Board } from '../../../types/board';
import { DndContext, DragEndEvent } from '@dnd-kit/core';
import { PieceColor } from '../../../types/piece';
import { getSquareName } from '../../../utils/square';
import { useRealtime } from '../../../contexts/realtime';
import { Move } from '../../../types/move';
import _ from 'lodash';
import {
  determineMoveType,
  getCastlingMove,
  isOnPromotionRow,
} from '../../../utils/board';

interface ChessBoardProps {
  board: Board;
  myColor: PieceColor;
  disableDrag: true | PieceColor;
  setWaitingForMoveResponse: (optimisticMove: Move) => void;
  setWaitingForPromotionChoice: (move: Move) => void;
  optimisticMove?: Move;
}

export const Chessboard: FC<ChessBoardProps> = ({
  board,
  myColor,
  disableDrag,
  setWaitingForMoveResponse,
  setWaitingForPromotionChoice,
  optimisticMove,
}) => {
  const { sendMessage } = useRealtime();

  const optimisticBoard = useMemo(() => {
    const newBoard = _.cloneDeep(board);
    if (optimisticMove) {
      newBoard.state[optimisticMove.destIndex] =
        newBoard.state[optimisticMove.srcIndex];
      newBoard.state[optimisticMove.srcIndex] = null;

      if (optimisticMove.promotion) {
        newBoard.state[optimisticMove.destIndex]!.piece_type =
          optimisticMove.promotion;
      }

      if (
        optimisticMove.moveType === 'KingsideCastling' ||
        optimisticMove.moveType === 'QueensideCastling'
      ) {
        const castlingMove = getCastlingMove(myColor, optimisticMove.moveType);

        newBoard.state[castlingMove.rookDest] =
          newBoard.state[castlingMove.rookSrc];
        newBoard.state[castlingMove.rookSrc] = null;
      }
    }

    return newBoard;
  }, [board, optimisticMove, myColor]);

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
        {optimisticBoard.state.map((piece, i) => (
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
