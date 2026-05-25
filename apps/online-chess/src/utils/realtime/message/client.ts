import { MoveDTO } from '../../../types/move';

export type ClientMessage =
  | {
      type: 'JoinGame';
    }
  | {
      type: 'PlayerMove';
      data: MoveDTO;
    };
