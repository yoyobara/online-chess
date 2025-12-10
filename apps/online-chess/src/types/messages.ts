// Client message types (sent from frontend to backend)
export type ClientMessage =
  | { type: 'LookingForMatch' }
  | { type: 'ConnectionClosed' };

// Server message types (received from backend)
export type ServerMessage =
  | { type: 'Log'; message: string }
  | { type: 'WaitingForMatch' }
  | { type: 'MatchFound'; opponent_name: string; you_are_white: boolean }
  | { type: 'OpponentDisconnected' };
