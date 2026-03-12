import { FC, useCallback, useEffect, useReducer } from 'react';
import { useRealtime } from '../../contexts/realtime';
import { PlayPage } from './PlayPage';
import { PieceType } from '../../types/piece';
import { Move } from '../../types/move';
import { getSquareName } from '../../utils/square';
import { gameStateReducer } from '../../reducers/game_state';

export const PlayPageContainer: FC = () => {
  const { lastMessage, sendMessage } = useRealtime();

  const [gameState, dispatch] = useReducer(gameStateReducer, null);

  useEffect(() => {
    console.log('current state', gameState);
  }, [gameState]);

  useEffect(() => {
    console.log('recieved', lastMessage);
    if (lastMessage) dispatch(lastMessage);
  }, [lastMessage]);

  const setWaitingForMoveResponse = useCallback((optimisticMove: Move) => {
    dispatch({
      type: 'WaitingForMoveResponse',
      move: optimisticMove,
    });
  }, []);

  const setWaitingForPromotionChoice = useCallback((move: Move) => {
    dispatch({
      type: 'WaitingForPromotionChoice',
      move,
    });
  }, []);

  const onPromotionModalClose = useCallback(() => {
    dispatch({
      type: 'PromotionModalClose',
    });
  }, []);

  const onPromotionModalSelect = useCallback(
    (pieceType: PieceType) => {
      if (gameState?.type !== 'WaitForPromotionChoice') {
        throw Error('not waiting for promotion choice..');
      }

      sendMessage({
        type: 'PlayerMove',
        data: {
          src_square: getSquareName(gameState.optimisticMove.srcIndex),
          dest_square: getSquareName(gameState.optimisticMove.destIndex),
          promotion: pieceType,
          move_type: gameState.optimisticMove.moveType,
        },
      });

      setWaitingForMoveResponse({
        ...gameState.optimisticMove,
        promotion: pieceType,
      });
    },
    [gameState, sendMessage, setWaitingForMoveResponse]
  );

  if (gameState === null) {
    return null;
  }

  return (
    <PlayPage
      gameState={gameState}
      setWaitingForMoveResponse={setWaitingForMoveResponse}
      setWaitingForPromotionChoice={setWaitingForPromotionChoice}
      onPromotionModalClose={onPromotionModalClose}
      onPromotionModalSelect={onPromotionModalSelect}
    />
  );
};
