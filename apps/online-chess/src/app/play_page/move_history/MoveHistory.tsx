import { FC } from 'react';
import { Paper } from '../../../components/Paper/Paper';
import styles from './MoveHistory.module.scss';
import { Move } from '../../../types/move';
import { getSquareName } from '../../../utils/square';

interface MoveHistoryProps {
  moves: Move[];
  className?: string;
}

const moveToText = (mv: Move): string => {
  const src = getSquareName(mv.srcIndex);
  const dest = getSquareName(mv.destIndex);

  return `${src} ➔ ${dest}`;
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
