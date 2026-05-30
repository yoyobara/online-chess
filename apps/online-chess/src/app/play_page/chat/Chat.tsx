import { FC, useEffect, useRef } from 'react';
import { Paper } from '../../../components/Paper/Paper';
import styles from './Chat.module.scss';
import { MessageBubble } from './message_bubble/MessageBubble';

export type Message = {
  id: number;
  authorId: number;
  content: string;
};

export interface ChatProps {
  className?: string;
  messages: Message[];
  userId: number;
  onSend: (msg: string) => void;
}

export const Chat: FC<ChatProps> = ({ className, messages, userId }) => {
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  return (
    <Paper className={`${styles.container} ${className}`}>
      <div className={styles.scroller}>
        {messages.map((msg) => (
          <MessageBubble
            key={msg.id}
            content={msg.content}
            isOwn={msg.authorId === userId}
          />
        ))}
        <div ref={bottomRef} />
      </div>
    </Paper>
  );
};
