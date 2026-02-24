import { Board } from './board';
import { PieceColor } from './piece';

export type MatchResult = { type: 'Draw' } | { type: 'Win'; data: PieceColor };

export type MatchState = {
  board: Board;
  move_count: number;
  match_result: MatchResult | null;
};
