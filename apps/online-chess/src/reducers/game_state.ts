import { Reducer } from 'react';
import { GameState, GameStateAction } from '../types/game_state';
import { invertColor } from '../utils/piece';
import { applyMove } from '../utils/board';

export const gameStateReducer: Reducer<GameState | null, GameStateAction> = (
  state: GameState | null,
  action: GameStateAction
) => {
  switch (action.type) {
    case 'GameInit': {
      if (state) {
        console.error('got GameInit action while already playing');
        return state;
      }

      const { initialState, color, opponentId } = action;

      return {
        type: 'Playing',
        game: {
          currentBoard: initialState.board,
          serverBoard: initialState.board,
          moveCount: initialState.move_count,
          myColor: color,
          opponentColor: invertColor(color),
          opponentId: opponentId,
        },
      };
    }

    case 'BoardUpdate': {
      if (
        state?.type !== 'Playing' &&
        state?.type !== 'WaitForMoveResponse' &&
        state?.type !== 'WaitForPromotionChoice'
      ) {
        console.error('got BoardUpdate action while not playing');
        return state;
      }

      const { board, move_count, match_result } = action.state;

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

    case 'ServerMoveResult':
      if (state?.type !== 'WaitForMoveResponse') {
        console.error(
          'got ServerMoveResult action not when waiting for response'
        );
        return state;
      }

      if (!action.success) {
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
      if (
        state?.type !== 'Playing' &&
        state?.type !== 'WaitForPromotionChoice'
      ) {
        console.error('not currently playing..');
        return state;
      }

      return {
        type: 'WaitForMoveResponse',
        game: {
          ...state.game,
          currentBoard: applyMove(state.game.serverBoard, action.move),
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
          currentBoard: applyMove(state.game.serverBoard, action.move),
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
