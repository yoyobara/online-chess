import { MatchState } from '../../../types/match';
import { Move, MoveType } from '../../../types/move';
import { PieceColor, PieceType } from '../../../types/piece';

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
      type: 'PlayerMove';
      data: [
        {
          src_square: string;
          dest_square: string;
          promotion: PieceType | null;
          move_type: MoveType;
        },
        MatchState
      ];
    };
