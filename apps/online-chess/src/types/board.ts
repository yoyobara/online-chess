import { Piece } from './piece';

export type CastlingRights = {
  queenside: boolean;
  kingside: boolean;
};

export type Board = {
  state: Array<Piece | null>;
  castling_rights: [CastlingRights, CastlingRights];
};
