import { Move } from '../../../types/move';

export type ClientMessage =
  | {
      type: 'JoinGame';
    }
  | {
      type: 'PlayerMove';
      data: Move;
    }
  | {
      type: 'ChatMessage';
      data: string;
    };
