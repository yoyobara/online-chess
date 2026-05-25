import { Board } from './board';
import { PieceColor } from './piece';

export type MatchResult = { type: 'Draw' } | { type: 'Win'; data: PieceColor };

export type MatchState = {
  board: Board;
  current_turn: PieceColor;
  match_result: MatchResult | null;
};
