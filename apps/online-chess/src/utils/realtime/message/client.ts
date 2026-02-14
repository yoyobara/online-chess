import { PieceType } from '../../../types/piece';

export type ClientMessage =
  | {
      type: 'JoinGame';
    }
  | {
      type: 'PlayerMove';
      data: {
        src_square: string;
        dest_square: string;
        captured_piece: PieceType | null;
      };
    };
