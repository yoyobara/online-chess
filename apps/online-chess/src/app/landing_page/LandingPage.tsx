import { FC } from 'react';
import styles from './LandingPage.module.scss';
import { Button } from '../../components/Button/Button';
import { Outlet, useNavigate } from 'react-router-dom';

export const LandingPage: FC = (props) => {
  const navigate = useNavigate();

  return (
    <>
      <div className={styles.landing_page}>
        <div className={styles.content}>
          <div className={styles.title}>Play Chess Online</div>
          <div className={styles.subtitle}>
            Challenge friends or test your skills against players worldwide.
          </div>
          <div className={styles.buttons}>
            <Button variant="black" onClick={() => navigate('/sign_in')}>
              Sign In
            </Button>
            <Button variant="white" onClick={() => navigate('/sign_up')}>
              Sign Up
            </Button>
          </div>
        </div>
      </div>
      <Outlet />
    </>
  );
};
