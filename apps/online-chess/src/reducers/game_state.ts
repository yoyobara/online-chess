import { Reducer } from 'react';
import { GameState, GameStateAction } from '../types/game_state';
import { invertColor } from '../utils/piece';

export const gameStateReducer: Reducer<GameState | null, GameStateAction> = (
  state: GameState | null,
  action: GameStateAction
) => {
  switch (action.type) {
    case 'JoinResponse': {
      if (state) {
        throw Error('got JoinResponse message while already playing');
      }

      const { initial_state, color, opponent_id } = action.data;

      return {
        type: 'Playing',
        game: {
          currentBoard: initial_state.board,
          moveCount: initial_state.move_count,
          myColor: color,
          opponentColor: invertColor(color),
          opponentId: opponent_id,
        },
      };
    }

    case 'NewState': {
      if (state?.type !== 'Playing') {
        throw Error('got NewState message while not playing');
      }

      const { board, move_count, match_result } = action.data;

      if (!match_result) {
        return {
          type: 'Playing',
          game: {
            ...state.game,
            currentBoard: board,
            moveCount: move_count,
          },
        };
      } else {
        return {
          type: 'Ended',
          game: {
            ...state.game,
            currentBoard: board,
            moveCount: move_count,
          },
          result: match_result,
        };
      }
    }

    case 'MoveResult':
      if (state?.type !== 'WaitForMoveResponse') {
        throw Error('got bad move message not when waiting for response');
      }

      if (!action.data) {
        return {
          type: 'Playing',
          game: state.game,
        };
      } else {
        return state;
      }

    case 'WaitingForMoveResponse':
      if (state?.type !== 'Playing' && state?.type !== 'WaitForPromotionChoice')
        throw Error('not currently playing..');

      return {
        type: 'WaitForMoveResponse',
        game: state.game,
        optimisticMove: action.move,
      };

    case 'WaitingForPromotionChoice':
      if (state?.type !== 'Playing') throw Error('not currently playing..');
      return {
        type: 'WaitForPromotionChoice',
        game: state.game,
        optimisticMove: action.move,
      };

    case 'PromotionModalClose':
      if (state?.type !== 'WaitForPromotionChoice')
        throw Error('not waiting for promotion choice..');

      return {
        type: 'Playing',
        game: state.game,
      };

    default:
      console.log(action);
      return state;
  }
};
