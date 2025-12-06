import { FC } from 'react';
import { Modal } from '../../components/Modal/Modal';
import styles from './MatchmakingModal.module.scss';
import loading_spinner from '../../assets/loading_spinner.svg';

export const MatchmakingModal: FC = () => {
  return (
    <Modal modalClassName={styles.matchmaking_modal}>
      <div>Looking For Match...</div>
      <img className={styles.loading_spinner} src={loading_spinner} alt="..." />
    </Modal>
  );
};
