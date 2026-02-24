import { Board } from './board';
import { PieceColor } from './piece';

export type PlayerStatus = 'win' | 'lose' | 'draw';

export type GameData = {
  currentBoard: Board;
  myColor: PieceColor;
  moveCount: number;
  opponentId: number;
};

export type GameState =
  | { type: 'NotJoined' }
  | { type: 'Playing'; game: GameData }
  | {
      type: 'Ended';
      game: GameData;
      myStatus: PlayerStatus;
      opponentStatus: PlayerStatus;
    };
