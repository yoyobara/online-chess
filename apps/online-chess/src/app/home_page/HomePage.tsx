import { FC } from 'react';
import styles from './HomePage.module.scss';
import { Button } from '../../components/Button/Button';
import { useNavigate } from 'react-router-dom';
import { Paper } from '../../components/Paper/Paper';

export const HomePage: FC = () => {
  const navigate = useNavigate();

  return (
    <div className={styles.home_page}>
      <Paper>
        <button>yay</button>
      </Paper>
    </div>
  );
};
