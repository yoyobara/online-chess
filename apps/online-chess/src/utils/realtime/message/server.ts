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
      data: boolean;
    }
  | {
      type: 'NewState';
      data: MatchState;
    };
