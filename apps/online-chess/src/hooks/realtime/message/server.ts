import { MatchState } from '../../../types/match';
import { PieceColor } from '../../../types/piece';

type JoinResponse = {
  initial_state: MatchState;
  color: PieceColor;
};

export type ServerMessage = {
  type: 'JoinResponse';
  data: JoinResponse;
};
