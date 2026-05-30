import { ChatMessage } from './chat';
import { Board } from './board';
import { MatchResult, MatchState } from './match';
import { Move } from './move';
import { PieceColor } from './piece';

export type GameData = {
  currentBoard: Board;
  serverBoard: Board;
  currentTurn: PieceColor;
  myColor: PieceColor;
  opponentColor: PieceColor;
  opponentId: number;
  moveList: Move[];
  messages: ChatMessage[];
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
  | {
      type: 'GameInit';
      initialState: MatchState;
      color: PieceColor;
      opponentId: number;
      initialMoves: Move[];
    }
  | { type: 'ServerMoveResult'; success: boolean }
  | { type: 'BoardUpdate'; move: Move; newState: MatchState }
  | { type: 'WaitingForMoveResponse'; move: Move }
  | { type: 'WaitingForPromotionChoice'; move: Move }
  | { type: 'PromotionModalClose' }
  | { type: 'ChatMessage'; message: ChatMessage };
