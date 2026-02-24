import { FC, useEffect, useState } from 'react';
import { useRealtime } from '../../contexts/realtime';
import { PlayPage } from './PlayPage';
import { GameState, PlayerStatus } from '../../types/game_state';
import { MatchResult } from '../../types/match';
import { PieceColor } from '../../types/piece';

const determinePlayerStatus = (
  myColor: PieceColor,
  res: MatchResult
): PlayerStatus => {
  switch (res.type) {
    case 'Draw':
      return 'draw';

    case 'Win':
      return res.data === myColor ? 'win' : 'lose';
  }
};

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

          console.log(lastMessage.data.match_result);

          if (lastMessage.data.match_result === null) {
            return {
              type: 'Playing',
              game: {
                ...prev.game,
                currentBoard: lastMessage.data.board,
                moveCount: lastMessage.data.move_count,
              },
            };
          } else {
            return {
              type: 'Ended',
              game: {
                ...prev.game,
                currentBoard: lastMessage.data.board,
                moveCount: lastMessage.data.move_count,
              },
              myStatus: determinePlayerStatus(
                prev.game.myColor,
                lastMessage.data.match_result
              ),
              opponentStatus: determinePlayerStatus(
                prev.game.myColor === 'White' ? 'Black' : 'White',
                lastMessage.data.match_result
              ),
            };
          }
        });
        break;

      case 'MoveResult':
        break;
    }
  }, [lastMessage]);

  if (gameState.type === 'NotJoined') {
    return null;
  }

  return <PlayPage gameState={gameState} />;
};
