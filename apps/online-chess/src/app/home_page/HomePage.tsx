import { FC, useEffect, useState } from 'react';
import styles from './HomePage.module.scss';
import { Button } from '../../components/Button/Button';
import { useNavigate } from 'react-router-dom';
import { Paper } from '../../components/Paper/Paper';

import profile_default from '../../assets/profile_default.svg';
import trophy_icon from '../../assets/trophy.svg';
import play_circle from '../../assets/play_circle.svg';
import logout_icon from '../../assets/logout.svg';

import { useRequiredAuth } from '../../contexts/auth';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import useWebSocket from 'react-use-websocket';
import { MatchmakingModal } from '../matchmaking_modal/MatchmakingModal';

export const HomePage: FC = () => {
  const auth = useRequiredAuth();
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  const [matchmaking, setMatchmaking] = useState<boolean>(false);
  const { lastJsonMessage, lastMessage } = useWebSocket(
    '/matchmaking',
    undefined,
    matchmaking
  );

  useEffect(() => {
    if (lastJsonMessage) {
      navigate(`/play/${lastJsonMessage}`);
    }
  }, [lastJsonMessage, navigate]);

  useEffect(() => {
    console.log(lastMessage);
  }, [lastMessage]);

  const { data: rank } = useQuery<number>({
    queryKey: ['user_rank'],
    queryFn: () =>
      fetch('/api/user/rank', { credentials: 'include' }).then((resp) =>
        resp.json()
      ),
  });

  const { mutate } = useMutation({
    mutationFn: () =>
      fetch('/api/auth/logout', { method: 'POST', credentials: 'include' }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['auth_data'] });
      navigate('/');
    },
  });

  return (
    <div className={styles.home_page}>
      <Paper className={styles.main_paper}>
        <img className={styles.profile_photo} src={profile_default} alt="" />
        <div className={styles.username}>{auth.username}</div>
        <Paper className={styles.rank_paper}>
          <img className={styles.trophy_icon} src={trophy_icon} alt="trophy" />
          <div className={styles.rank_text}>{rank ?? '...'}</div>
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
          onClick={() => mutate()}
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
