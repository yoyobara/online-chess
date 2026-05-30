import { MatchState } from '../../../types/match';
import { Move } from '../../../types/move';
import { PieceColor } from '../../../types/piece';
import { ChatMessage } from '../../../types/chat';

type JoinResponse = {
  initial_state: MatchState;
  color: PieceColor;
  opponent_id: number;
  initial_moves: Move[];
  initial_chat_messages: ChatMessage[];
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
      data: [Move, MatchState];
    }
  | {
      type: 'ChatMessage';
      data: ChatMessage;
    };
