import { FC } from 'react';
import styles from './LandingPage.module.scss';
import { Button } from '../../components/Button/Button';

export const LandingPage: FC = (props) => {
  return (
    <div className={styles.landing_page}>
      <div className={styles.content}>
        <div className={styles.title}>Play Chess Online</div>
        <div className={styles.subtitle}>
          Challenge friends or test your skills against players worldwide.
        </div>
        <div className={styles.buttons}>
          <Button variant="black">Sign In</Button>
          <Button variant="white">Sign Up</Button>
        </div>
      </div>
    </div>
  );
};
