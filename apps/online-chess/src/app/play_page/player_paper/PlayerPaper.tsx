import { FC } from 'react';
import { Paper } from '../../../components/Paper/Paper';
import styles from './PlayerPaper.module.scss';

import profile_default from '../../../assets/profile_default.svg';

interface PlayPaperProps {
  className?: string;
  variant: 'white' | 'purple';
  playerName: string;
  playerRating: number;
}

export const PlayerPaper: FC<PlayPaperProps> = ({
  variant,
  playerName,
  playerRating,
  className,
}) => {
  return (
    <Paper className={`${styles.player_paper} ${styles[variant]} ${className}`}>
      <img className={styles.profile_photo} src={profile_default} alt="" />
      <div className={styles.player_name}>{playerName}</div>
      <div className={styles.player_rating}>Rating: {playerRating}</div>
    </Paper>
  );
};
