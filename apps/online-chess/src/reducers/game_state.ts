import { Reducer } from 'react';
import { GameState, GameStateAction } from '../types/game_state';
import { invertColor } from '../utils/piece';
import { applyMove } from '../utils/board';

export const gameStateReducer: Reducer<GameState | null, GameStateAction> = (
  state: GameState | null,
  action: GameStateAction
) => {
  switch (action.type) {
    case 'JoinResponse': {
      if (state) {
        console.error('got JoinResponse message while already playing');
        return state;
      }

      const { initial_state, color, opponent_id } = action.data;

      return {
        type: 'Playing',
        game: {
          currentBoard: initial_state.board,
          serverBoard: initial_state.board,
          moveCount: initial_state.move_count,
          myColor: color,
          opponentColor: invertColor(color),
          opponentId: opponent_id,
        },
      };
    }

    case 'NewState': {
      if (
        state?.type !== 'Playing' &&
        state?.type !== 'WaitForMoveResponse' &&
        state?.type !== 'WaitForPromotionChoice'
      ) {
        console.error('got NewState message while not playing');
        return state;
      }

      const { board, move_count, match_result } = action.data;

      if (!match_result) {
        return {
          type: 'Playing',
          game: {
            ...state.game,
            currentBoard: board,
            serverBoard: board,
            moveCount: move_count,
          },
        };
      } else {
        return {
          type: 'Ended',
          game: {
            ...state.game,
            currentBoard: board,
            serverBoard: board,
            moveCount: move_count,
          },
          result: match_result,
        };
      }
    }

    case 'MoveResult':
      if (state?.type !== 'WaitForMoveResponse') {
        console.error('got bad move message not when waiting for response');
        return state;
      }

      if (!action.data) {
        return {
          type: 'Playing',
          game: {
            ...state.game,
            currentBoard: state.game.serverBoard,
          },
        };
      } else {
        return state;
      }

    case 'WaitingForMoveResponse':
      if (state?.type !== 'Playing' && state?.type !== 'WaitForPromotionChoice') {
        console.error('not currently playing..');
        return state;
      }

      return {
        type: 'WaitForMoveResponse',
        game: {
          ...state.game,
          currentBoard: applyMove(
            state.game.serverBoard,
            action.move,
            state.game.myColor
          ),
        },
        optimisticMove: action.move,
      };

    case 'WaitingForPromotionChoice':
      if (state?.type !== 'Playing') {
        console.error('not currently playing..');
        return state;
      }

      return {
        type: 'WaitForPromotionChoice',
        game: {
          ...state.game,
          currentBoard: applyMove(
            state.game.serverBoard,
            action.move,
            state.game.myColor
          ),
        },
        optimisticMove: action.move,
      };

    case 'PromotionModalClose':
      if (state?.type !== 'WaitForPromotionChoice') {
        console.error('not waiting for promotion choice..');
        return state;
      }

      return {
        type: 'Playing',
        game: {
          ...state.game,
          currentBoard: state.game.serverBoard,
        },
      };

    default:
      console.log(action);
      return state;
  }
};
