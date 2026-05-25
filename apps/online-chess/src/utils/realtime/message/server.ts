import { MatchState } from '../../../types/match';
import { MoveDTO } from '../../../types/move';
import { PieceColor } from '../../../types/piece';

type JoinResponse = {
  initial_state: MatchState;
  color: PieceColor;
  opponent_id: number;
  initial_moves: MoveDTO[];
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
      type: 'PlayerMove';
      data: [MoveDTO, MatchState];
    };
