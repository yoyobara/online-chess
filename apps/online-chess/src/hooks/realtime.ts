import useWebSocket from 'react-use-websocket';

export const useRealtime = (matchId: string) => {
  const { lastJsonMessage, sendJsonMessage } = useWebSocket(
    `/join/${matchId}`,
    undefined
  );

  return { lastJsonMessage, sendJsonMessage };
};
