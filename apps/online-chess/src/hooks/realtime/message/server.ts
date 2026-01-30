import { MatchState } from '../../../types/match';
import { PieceColor } from '../../../types/piece';
import { UserData } from '../../../types/user';

type JoinResponse = {
  initial_state: MatchState;
  color: PieceColor;
  opponent: UserData;
};

export type ServerMessage = {
  type: 'JoinResponse';
  data: JoinResponse;
};
