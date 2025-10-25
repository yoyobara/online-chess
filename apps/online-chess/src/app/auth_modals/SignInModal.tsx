import { FC } from 'react';
import styles from './auth_modal.module.scss';
import { Modal } from '../../components/Modal/Modal';
import { Button } from '../../components/Button/Button';
import { Input } from '../../components/Input/Input';
import { Link, useNavigate } from 'react-router-dom';

export const SignInModal: FC = () => {
  const navigate = useNavigate();

  return (
    <Modal onOverlayClick={() => navigate('/')}>
      <div className={styles.title}>Sign In</div>
      <Input placeholder="Email" />
      <Input placeholder="Password" type="password" />
      <Button variant="black">Sign In</Button>
      <div className={styles.redirection_text}>
        Don't have an accout? <Link to="/sign_up">Sign Up</Link>
      </div>
    </Modal>
  );
};
