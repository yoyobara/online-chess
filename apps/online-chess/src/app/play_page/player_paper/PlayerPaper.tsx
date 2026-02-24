import { FC } from 'react';
import { Paper } from '../../../components/Paper/Paper';
import styles from './PlayerPaper.module.scss';

import profile_default from '../../../assets/profile_default.svg';
import { PlayerStatus } from '../../../types/game_state';

interface PlayPaperProps {
  className?: string;
  variant: 'white' | 'purple';
  playerName: string | null;
  playerRating: number | null;
  status?: PlayerStatus;
}

const STATUS_TO_EMOJI: Record<PlayerStatus, string> = {
  win: '🏆',
  lose: '🤕',
  draw: '🟰',
};

export const PlayerPaper: FC<PlayPaperProps> = ({
  variant,
  playerName,
  playerRating,
  status,
  className,
}) => {
  return (
    <Paper className={`${styles.player_paper} ${styles[variant]} ${className}`}>
      <img className={styles.profile_photo} src={profile_default} alt="" />
      <div className={styles.player_name}>
        {playerName ?? '...'}
        {status && STATUS_TO_EMOJI[status]}
      </div>
      <div className={styles.player_rating}>
        Rating: {playerRating ?? '...'}
      </div>
    </Paper>
  );
};
