import { FC, useEffect, useRef, useState, KeyboardEvent } from 'react';
import { Paper } from '../../../components/Paper/Paper';
import { Input } from '../../../components/Input/Input';
import { Button } from '../../../components/Button/Button';
import styles from './Chat.module.scss';
import { MessageBubble } from './message_bubble/MessageBubble';
import { ChatMessage } from '../../../types/game_state';

export interface ChatProps {
  className?: string;
  messages: ChatMessage[];
  userId: number;
  onSend: (msg: string) => void;
}

export const Chat: FC<ChatProps> = ({
  className,
  messages,
  userId,
  onSend,
}) => {
  const [inputValue, setInputValue] = useState('');
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const handleSend = () => {
    if (inputValue.trim()) {
      onSend(inputValue.trim());
      setInputValue('');
    }
  };

  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      handleSend();
    }
  };

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
      <div className={styles.input_area}>
        <Input
          placeholder="Type a message..."
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
          onKeyDown={handleKeyDown}
          className={styles.input}
        />
        <Button
          onClick={handleSend}
          className={styles.send_button}
          variant="purple"
        >
          Send
        </Button>
      </div>
    </Paper>
  );
};
