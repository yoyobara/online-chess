import { FC } from 'react';
import styles from './auth_modal.module.scss';
import { Modal } from '../../components/Modal/Modal';
import { Button } from '../../components/Button/Button';
import { Input } from '../../components/Input/Input';
import { Link } from 'react-router-dom';

export const SignUpModal: FC = () => {
  return (
    <Modal>
      <div className={styles.title}>Sign Up</div>
      <Input placeholder="Username" />
      <Input placeholder="Email" />
      <Input placeholder="Password" type="password" />
      <Button variant="black">Sign Up</Button>
      <div className={styles.redirection_text}>
        Already have an account? <Link to="/sign_in">Sign In</Link>
      </div>
    </Modal>
  );
};
