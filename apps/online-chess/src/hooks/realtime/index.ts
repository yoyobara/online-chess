import { useEffect } from 'react';
import useWebSocket, { ReadyState } from 'react-use-websocket';
import { ServerMessage } from './message';

export const useRealtime = (matchId: string) => {
  const { lastJsonMessage, sendJsonMessage, readyState } =
    useWebSocket<ServerMessage | null>(`/join/${matchId}`, undefined);

  useEffect(() => {
    if (readyState === ReadyState.OPEN)
      sendJsonMessage({
        type: 'JoinGame',
      });
  }, [sendJsonMessage, readyState]);

  return { lastJsonMessage, sendJsonMessage };
};
