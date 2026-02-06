import { MatchState } from '../../../types/match';
import { PieceColor } from '../../../types/piece';

type JoinResponse = {
  initial_state: MatchState;
  color: PieceColor;
  opponent_id: number;
};

export type ServerMessage =
  | {
      type: 'JoinResponse';
      data: JoinResponse;
    }
  | {
      type: 'MoveResult';
      data: MatchState | null;
    }
  | {
      type: 'NewState';
      data: MatchState;
    };
