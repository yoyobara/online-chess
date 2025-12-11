import {
  createContext,
  FC,
  PropsWithChildren,
  useCallback,
  useContext,
} from 'react';
import useWebSocket, { ReadyState } from 'react-use-websocket';
import { ClientMessage, ServerMessage } from '../types/messages';

interface RealtimeCommunication {
  lastServerMessage: ServerMessage | null;
  sendMessage: (message: ClientMessage) => void;
  readyState: ReadyState;
}

const realtimeContext = createContext<RealtimeCommunication | null>(null);

export const RealtimeProvider: FC<PropsWithChildren> = ({ children }) => {
  const { lastJsonMessage, sendJsonMessage, readyState } =
    useWebSocket('/realtime');

  // TODO switch with real validation
  const lastServerMessage = lastJsonMessage as ServerMessage | null;

  const sendMessage = useCallback(
    (message: ClientMessage) => {
      sendJsonMessage(message);
    },
    [sendJsonMessage]
  );

  return (
    <realtimeContext.Provider
      value={{
        lastServerMessage,
        sendMessage,
        readyState,
      }}
    >
      {children}
    </realtimeContext.Provider>
  );
};

export const useRealtime = () => {
  const context = useContext(realtimeContext);

  if (!context) {
    throw Error('realtime context has not been initialized');
  }

  return context;
};
