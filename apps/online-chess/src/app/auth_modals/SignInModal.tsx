import { FC } from 'react';
import styles from './auth_modal.module.scss';
import { Modal } from '../../components/Modal/Modal';
import { Button } from '../../components/Button/Button';
import { Input } from '../../components/Input/Input';
import { Link, useNavigate } from 'react-router-dom';
import { SubmitHandler, useForm } from 'react-hook-form';
import { SignInInputs } from '../../types/auth/sign_in';
import { useSignIn } from '../../queries/auth/sign_in';

export const SignInModal: FC = () => {
  const navigate = useNavigate();

  const {
    register,
    handleSubmit,
    setError,
    formState: { errors },
  } = useForm<SignInInputs>();

  const { loginMutation } = useSignIn(setError);

  const onSubmit: SubmitHandler<SignInInputs> = (data) =>
    loginMutation.mutate(data);

  return (
    <Modal modalClassName={styles.form} onOverlayClick={() => navigate('/')}>
      <div className={styles.title}>Sign In</div>
      <Input
        placeholder="Email"
        type="email"
        {...register('email', { required: 'please enter a valid email' })}
      />
      {errors.email && (
        <span className={styles.error_text}>{errors.email.message}</span>
      )}
      <Input
        placeholder="Password"
        type="password"
        {...register('password', { required: 'please enter a valid password' })}
      />
      {errors.password && (
        <span className={styles.error_text}>{errors.password.message}</span>
      )}
      <Button variant="black" onClick={handleSubmit(onSubmit)}>
        Sign In
      </Button>
      {errors.root && (
        <span className={styles.error_text}>{errors.root.message}</span>
      )}
      <div className={styles.redirection_text}>
        Don't have an accout? <Link to="/sign_up">Sign Up</Link>
      </div>
    </Modal>
  );
};
