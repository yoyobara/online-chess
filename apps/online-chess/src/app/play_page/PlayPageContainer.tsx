import { FC, useCallback, useEffect, useState } from 'react';
import { useRealtime } from '../../contexts/realtime';
import { PlayPage } from './PlayPage';
import { GameState, PlayerStatus } from '../../types/game_state';
import { MatchResult } from '../../types/match';
import { PieceColor } from '../../types/piece';
import { Move } from '../../types/move';

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

  const [gameState, setGameState] = useState<GameState | null>(null);

  useEffect(() => {
    console.log('state: ', gameState?.type);
  }, [gameState]);

  useEffect(() => {
    if (!lastMessage) return;
    console.log('got message: ', lastMessage);

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
          if (!prev || prev.type === 'Ended') {
            throw Error('got NewState message while not playing');
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
        if (!lastMessage.data) {
          setGameState((prev) => {
            if (prev?.type !== 'WaitForMoveResponse') {
              throw Error('got bad move message not when playing');
            }

            return {
              type: 'Playing',
              game: prev.game,
            };
          });
        }
    }
  }, [lastMessage]);

  const setWaitingForMoveResponse = useCallback((optimisticMove: Move) => {
    setGameState((prev) => {
      if (prev?.type !== 'Playing') throw Error('not currently playing..');

      return {
        ...prev,
        type: 'WaitForMoveResponse',
        optimisticMove,
      };
    });
  }, []);

  if (gameState === null) {
    return null;
  }

  return (
    <PlayPage
      gameState={gameState}
      setWaitingForMoveResponse={setWaitingForMoveResponse}
    />
  );
};
