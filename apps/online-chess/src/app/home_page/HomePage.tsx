import { FC } from 'react';
import styles from './HomePage.module.scss';
import { Button } from '../../components/Button/Button';
import { useNavigate } from 'react-router-dom';
import { Paper } from '../../components/Paper/Paper';

import profile_default from '../../assets/profile_default.svg';
import trophy_icon from '../../assets/trophy.svg';
import play_circle from '../../assets/play_circle.svg';
import logout_icon from '../../assets/logout.svg';

import { useRequiredAuth } from '../../contexts/auth';
import { MatchmakingModal } from '../matchmaking_modal/MatchmakingModal';
import { useMatchmaking } from '../../hooks/matchmaking';
import { useLogout } from '../../queries/auth/logout';

export const HomePage: FC = () => {
  const me = useRequiredAuth();
  const navigate = useNavigate();

  const { matchmaking, setMatchmaking } = useMatchmaking((matchId) =>
    navigate(`/play/${matchId}`)
  );

  const { logoutMutation } = useLogout();

  return (
    <div className={styles.home_page}>
      <Paper className={styles.main_paper}>
        <img className={styles.profile_photo} src={profile_default} alt="" />
        <div className={styles.username}>{me.username}</div>
        <Paper className={styles.rank_paper}>
          <img className={styles.trophy_icon} src={trophy_icon} alt="trophy" />
          <div className={styles.rank_text}>{me.rank ?? '...'}</div>
        </Paper>
        <Button
          variant="purple"
          className={styles.play_button}
          onClick={() => setMatchmaking(true)}
        >
          <img height="100%" src={play_circle} alt=""></img>
          Play Online!
        </Button>
        <Button
          onClick={() => logoutMutation.mutate()}
          variant="white"
          className={styles.logout_button}
        >
          <img src={logout_icon} alt="logout" />
        </Button>
      </Paper>
      {matchmaking && <MatchmakingModal />}
    </div>
  );
};
