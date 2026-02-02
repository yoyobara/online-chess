import {
  createContext,
  FC,
  PropsWithChildren,
  useCallback,
  useContext,
  useEffect,
} from 'react';
import useWebSocket, { ReadyState } from 'react-use-websocket';
import { ClientMessage, ServerMessage } from '../utils/realtime/message';

export interface Realtime {
  lastMessage: ServerMessage | null;
  sendMessage: (msg: ClientMessage) => void;
}

const realtimeContext = createContext<Realtime | undefined>(undefined);

export const RealtimeProvider: FC<PropsWithChildren & { matchId: string }> = ({
  children,
  matchId,
}) => {
  const { lastJsonMessage, sendJsonMessage, readyState } =
    useWebSocket<ServerMessage | null>(`/join/${matchId}`, undefined);

  useEffect(() => {
    if (readyState === ReadyState.OPEN)
      sendJsonMessage({
        type: 'JoinGame',
      });
  }, [sendJsonMessage, readyState]);

  const sendMessage = useCallback(
    (msg: ClientMessage) => {
      sendJsonMessage(msg);
    },
    [sendJsonMessage]
  );

  return (
    <realtimeContext.Provider
      value={{ lastMessage: lastJsonMessage, sendMessage }}
    >
      {children}
    </realtimeContext.Provider>
  );
};

export const useRealtime = () => {
  const context = useContext(realtimeContext);

  if (context === undefined) {
    throw new Error('useRealtime must be used within RealtimeProvider');
  }

  return context;
};
