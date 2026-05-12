import { ServerMessage } from '../utils/realtime/message';
import { Board } from './board';
import { MatchResult } from './match';
import { Move } from './move';
import { PieceColor } from './piece';

export type GameData = {
  currentBoard: Board;
  serverBoard: Board;
  moveCount: number;
  myColor: PieceColor;
  opponentColor: PieceColor;
  opponentId: number;
};

export type GameState =
  | { type: 'Playing'; game: GameData }
  | {
      type: 'WaitForMoveResponse';
      game: GameData;
      optimisticMove: Move;
    }
  | {
      type: 'WaitForPromotionChoice';
      game: GameData;
      optimisticMove: Move;
    }
  | {
      type: 'Ended';
      game: GameData;
      result: MatchResult;
    };

export type GameStateAction =
  | ServerMessage
  | { type: 'WaitingForMoveResponse'; move: Move }
  | { type: 'WaitingForPromotionChoice'; move: Move }
  | { type: 'PromotionModalClose' };
