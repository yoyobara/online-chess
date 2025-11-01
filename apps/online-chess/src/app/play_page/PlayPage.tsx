import { FC } from 'react';
import styles from './PlayPage.module.scss';
import { Button } from '../../components/Button/Button';
import { Paper } from '../../components/Paper/Paper';

export const PlayPage: FC = () => {
  return (
    <div className={styles.play_page}>
      <div className={styles.board_container}></div>
      <Paper className={styles.chat}></Paper>
      <Paper className={styles.history}></Paper>
      <Paper className={styles.player}></Paper>
      <Paper className={styles.opponent}></Paper>
      <div className={styles.buttons}>
        <Button variant="black">Resign</Button>
        <Button variant="white">Offer Draw</Button>
      </div>
    </div>
  );
};
