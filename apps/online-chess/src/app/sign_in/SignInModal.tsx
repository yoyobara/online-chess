import { FC } from 'react';
import styles from './SignInModal.module.scss';
import { Modal } from '../../components/Modal/Modal';
import { Button } from '../../components/Button/Button';
import { Input } from '../../components/Input/Input';

export const SignInModal: FC = () => {
  return (
    <Modal>
      <div className={styles.title}>Sign In</div>
      <Input placeholder="Email" />
      <Input placeholder="Password" type="password" />
      <Button variant="black">Sign In</Button>
    </Modal>
  );
};
