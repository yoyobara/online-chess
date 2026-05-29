import { FC } from 'react';
import { Paper } from '../../../components/Paper/Paper';
import styles from './Chat.module.scss';

export interface ChatProps {
  className?: string;
}

export const Chat: FC<ChatProps> = ({ className }) => {
  return <Paper className={`${styles.container} ${className}`}></Paper>;
};
