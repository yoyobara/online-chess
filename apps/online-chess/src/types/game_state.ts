import { Board } from './board';
import { Move } from './move';
import { PieceColor } from './piece';

export type PlayerStatus = 'win' | 'lose' | 'draw';

export type GameData = {
  currentBoard: Board;
  myColor: PieceColor;
  moveCount: number;
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
      type: 'Ended';
      game: GameData;
      myStatus: PlayerStatus;
      opponentStatus: PlayerStatus;
    };
