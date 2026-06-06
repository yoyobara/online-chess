import {
  createContext,
  FC,
  PropsWithChildren,
  useCallback,
  useContext,
  useEffect,
  useRef,
} from 'react';
import useWebSocket, { ReadyState } from 'react-use-websocket';
import { ClientMessage, ServerMessage } from '../utils/realtime/message';

type MessageHandler<T extends ServerMessage['type']> = (
  data: Extract<ServerMessage, { type: T }>['data']
) => void;

export interface Realtime {
  sendMessage: (msg: ClientMessage) => void;
  subscribe: <T extends ServerMessage['type']>(
    type: T,
    handler: MessageHandler<T>
  ) => () => void;
}

const realtimeContext = createContext<Realtime | undefined>(undefined);

export const RealtimeProvider: FC<PropsWithChildren & { matchId: string }> = ({
  children,
  matchId,
}) => {
  const handlers = useRef<Map<string, Set<MessageHandler<any>>>>(new Map());

  const { lastJsonMessage, sendJsonMessage, readyState } =
    useWebSocket<ServerMessage | null>(`/join/${matchId}`, {
      onMessage: (event) => {
        const message: ServerMessage = JSON.parse(event.data);
        const typeHandlers = handlers.current.get(message.type);
        if (typeHandlers) {
          typeHandlers.forEach((handler) => handler(message.data));
        }
      },
      shouldReconnect: () => true,
    });

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

  const subscribe = useCallback(
    <T extends ServerMessage['type']>(type: T, handler: MessageHandler<T>) => {
      if (!handlers.current.has(type)) {
        handlers.current.set(type, new Set());
      }
      handlers.current.get(type)!.add(handler);

      return () => {
        const typeHandlers = handlers.current.get(type);
        if (typeHandlers) {
          typeHandlers.delete(handler);
        }
      };
    },
    []
  );

  return (
    <realtimeContext.Provider value={{ sendMessage, subscribe }}>
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

export const useRealtimeMessage = <T extends ServerMessage['type']>(
  type: T,
  handler: MessageHandler<T>
) => {
  const { subscribe } = useRealtime();

  useEffect(() => {
    return subscribe(type, handler);
  }, [subscribe, type, handler]);
};
