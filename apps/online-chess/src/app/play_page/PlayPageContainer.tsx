import { FC, useCallback, useReducer } from 'react';
import { useRealtime, useRealtimeMessage } from '../../contexts/realtime';
import { PlayPage } from './PlayPage';
import { PieceType } from '../../types/piece';
import { Move } from '../../types/move';
import { gameStateReducer } from '../../reducers/game_state';

export const PlayPageContainer: FC = () => {
  const { sendMessage } = useRealtime();
  const [gameState, dispatch] = useReducer(gameStateReducer, null);

  useRealtimeMessage('JoinResponse', (data) => {
    dispatch({
      type: 'GameInit',
      initialState: data.initial_state,
      color: data.color,
      opponentId: data.opponent_id,
      initialMoves: data.initial_moves,
      initialChatMessages: data.initial_chat_messages,
    });
  });

  useRealtimeMessage('MoveResult', (data) => {
    dispatch({
      type: 'ServerMoveResult',
      success: data,
    });
  });

  useRealtimeMessage('PlayerMove', (data) => {
    const [move, newState] = data;
    dispatch({
      type: 'BoardUpdate',
      move,
      newState,
    });
  });

  useRealtimeMessage('ChatMessage', (data) => {
    dispatch({
      type: 'ChatMessage',
      message: data,
    });
  });

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
