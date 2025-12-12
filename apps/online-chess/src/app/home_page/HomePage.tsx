import { FC } from 'react';
import styles from './HomePage.module.scss';
import { Button } from '../../components/Button/Button';
import { useNavigate } from 'react-router-dom';
import { Paper } from '../../components/Paper/Paper';

import profile_default from '../../assets/profile_default.svg';
import trophy_icon from '../../assets/trophy.svg';
import play_circle from '../../assets/play_circle.svg';

import { useRequiredAuth } from '../../contexts/auth';

export const HomePage: FC = () => {
  const auth = useRequiredAuth();

  return (
    <div className={styles.home_page}>
      <Paper className={styles.main_paper}>
        <img className={styles.profile_photo} src={profile_default} alt="" />
        <div className={styles.username}>{auth.username}</div>
        <Paper className={styles.rank_paper}>
          <img className={styles.trophy_icon} src={trophy_icon} alt="trophy" />
          <div className={styles.rank_text}>4234</div>
        </Paper>
        <Button variant="purple" className={styles.play_button}>
          <img height="100%" src={play_circle} alt=""></img>
          Play Online!
        </Button>
      </Paper>
    </div>
  );
};
