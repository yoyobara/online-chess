import { useEffect, useState } from 'react';
import useWebSocket from 'react-use-websocket';

export const useMatchmaking = (onMatchmaking: (matchId: string) => void) => {
  const [matchmaking, setMatchmaking] = useState<boolean>(false);
  const { lastJsonMessage } = useWebSocket(
    '/matchmaking',
    undefined,
    matchmaking
  );

  useEffect(() => {
    if (typeof lastJsonMessage == 'string') {
      onMatchmaking(lastJsonMessage);
    }
  }, [lastJsonMessage, onMatchmaking]);

  return { matchmaking, setMatchmaking };
};
