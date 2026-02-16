import { FC, useEffect, useState } from 'react';
import { useRealtime } from '../../contexts/realtime';
import { PlayPage } from './PlayPage';
import { GameState } from '../../types/game_state';

export const PlayPageContainer: FC = () => {
  const { lastMessage } = useRealtime();

  const [gameState, setGameState] = useState<GameState>({ type: 'NotJoined' });

  useEffect(() => {
    if (!lastMessage) return;

    switch (lastMessage.type) {
      case 'JoinResponse':
        setGameState({
          type: 'Playing',
          game: {
            currentBoard: lastMessage.data.initial_state.board,
            moveCount: lastMessage.data.initial_state.move_count,
            opponentId: lastMessage.data.opponent_id,
            myColor: lastMessage.data.color,
          },
        });
        break;

      case 'NewState':
        setGameState((prev) => {
          if (prev.type !== 'Playing') {
            throw Error('got NewState message not in Playing state');
          }

          return {
            ...prev,
            game: {
              ...prev.game,
              currentBoard: lastMessage.data.board,
              moveCount: lastMessage.data.move_count,
            },
          };
        });
        break;

      case 'MoveResult':
        break;
    }
  }, [lastMessage]);

  return gameState.type === 'Playing' ? (
    <PlayPage game={gameState.game} />
  ) : null;
};
