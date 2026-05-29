import { FC } from 'react';
import { Paper } from '../../../components/Paper/Paper';
import styles from './MoveHistory.module.scss';
import { Move } from '../../../types/move';

interface MoveHistoryProps {
  moves: Move[];
  className?: string;
}

const moveToText = (mv: Move): string => {
  return `${mv.src_square} ➔ ${mv.dest_square}`;
};

export const MoveHistory: FC<MoveHistoryProps> = ({ moves, className }) => {
  return (
    <Paper className={`${styles.move_history} ${className}`}>
      {moves.map((mv) => (
        <div className={styles.move}>{moveToText(mv)}</div>
      ))}
    </Paper>
  );
};
