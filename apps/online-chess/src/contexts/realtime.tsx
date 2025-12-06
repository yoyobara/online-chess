import { createContext, FC, PropsWithChildren, useContext } from 'react';
import useWebSocket, { SendMessage } from 'react-use-websocket';

interface RealtimeCommunication {
  lastMessage: any;
  sendMessage: SendMessage;
  readyState: any;
}

const realtimeContext = createContext<RealtimeCommunication | null>(null);

export const RealtimeProvider: FC<PropsWithChildren> = ({ children }) => {
  const { lastMessage, sendMessage, readyState } = useWebSocket('/realtime');

  return (
    <realtimeContext.Provider
      value={{ lastMessage: lastMessage?.data, sendMessage, readyState }}
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
