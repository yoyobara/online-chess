import { Move } from '../../../types/move';

export type ClientMessage =
  | {
      type: 'JoinGame';
    }
  | {
      type: 'PlayerMove';
      data: Move;
    };
