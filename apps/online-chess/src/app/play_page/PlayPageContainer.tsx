import { FC, useCallback, useEffect, useReducer } from 'react';
import { useRealtime } from '../../contexts/realtime';
import { PlayPage } from './PlayPage';
import { PieceType } from '../../types/piece';
import { Move } from '../../types/move';
import { gameStateReducer } from '../../reducers/game_state';

export const PlayPageContainer: FC = () => {
  const { lastMessage, sendMessage } = useRealtime();

  const [gameState, dispatch] = useReducer(gameStateReducer, null);

  useEffect(() => {
    if (!lastMessage) return;

    switch (lastMessage.type) {
      case 'JoinResponse':
        dispatch({
          type: 'GameInit',
          initialState: lastMessage.data.initial_state,
          color: lastMessage.data.color,
          opponentId: lastMessage.data.opponent_id,
          initialMoves: lastMessage.data.initial_moves,
          initialChatMessages: lastMessage.data.initial_chat_messages,
        });
        break;
      case 'MoveResult':
        dispatch({
          type: 'ServerMoveResult',
          success: lastMessage.data,
        });
        break;
      case 'PlayerMove': {
        const [move, newState] = lastMessage.data;

        dispatch({
          type: 'BoardUpdate',
          move,
          newState,
        });
        break;
      }
      case 'ChatMessage': {
        dispatch({
          type: 'ChatMessage',
          message: lastMessage.data,
        });
        break;
      }
    }
  }, [lastMessage]);

  const onSendMessage = useCallback(
    (content: string) => {
      sendMessage({
        type: 'ChatMessage',
        data: content,
      });
    },
    [sendMessage]
  );

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
          ...gameState.optimisticMove,
          promotion: pieceType,
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
      onSendMessage={onSendMessage}
    />
  );
};
