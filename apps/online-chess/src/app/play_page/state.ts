export type GameState =
  | { type: 'beforeMatch' }
  | {
      type: 'waitingForMatch';
    }
  | {
      type: 'ongoingMatch';
      opponentName: string;
    };
