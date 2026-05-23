import { FC } from 'react';
import { Paper } from '../../../components/Paper/Paper';
import styles from './MoveHistory.module.scss';
import { Move } from '../../../types/move';
import { getSquareName } from '../../../utils/square';

interface MoveHistoryProps {
  moves: Move[];
  className?: string;
}

export const MoveHistory: FC<MoveHistoryProps> = ({ moves, className }) => {
  console.log(moves);
  return (
    <Paper className={`${styles.move_history} ${className}`}>
      {moves.map((mv) => (
        <h1>{`${mv.moveType}: ${getSquareName(mv.srcIndex)} -> ${getSquareName(
          mv.destIndex
        )}${mv.promotion ? ' (' + mv.promotion + ')' : ''}`}</h1>
      ))}
    </Paper>
  );
};
