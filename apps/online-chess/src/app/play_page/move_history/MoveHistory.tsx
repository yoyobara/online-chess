import { FC } from 'react';
import { Paper } from '../../../components/Paper/Paper';
import styles from './MoveHistory.module.scss';
import { Move } from '../../../types/move';

interface MoveHistoryProps {
  moves: Move[];
  className?: string;
}

const moveToText = (mv: Move): string => {
  return `${mv.from} ➔ ${mv.to}`;
};

export const MoveHistory: FC<MoveHistoryProps> = ({ moves, className }) => {
  return (
    <Paper className={`${styles.container} ${className}`}>
      <div className={styles.title}>Moves</div>
      <hr style={{ width: '100%' }} />
      <div className={styles.scroller}>
        {moves.map((mv) => (
          <div className={styles.move}>{moveToText(mv)}</div>
        ))}
      </div>
    </Paper>
  );
};
