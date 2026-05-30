import { FC } from 'react';
import styles from './MessageBubble.module.scss';

export type MessageBubbleProps = {
  content: string;
  isOwn: boolean;
};

export const MessageBubble: FC<MessageBubbleProps> = ({ isOwn, content }) => {
  return (
    <div
      className={`${styles.message_bubble} ${
        isOwn ? styles.own : styles.opponent
      } `}
    >
      {content}
    </div>
  );
};
