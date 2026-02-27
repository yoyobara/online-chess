import { FC } from 'react';
import styles from './auth_modal.module.scss';
import { Modal } from '../../components/Modal/Modal';
import { Button } from '../../components/Button/Button';
import { Input } from '../../components/Input/Input';
import { Link, useNavigate } from 'react-router-dom';
import { SubmitHandler, useForm } from 'react-hook-form';
import { SignUpInputs } from '../../types/auth/sign_up';
import { useSignUp } from '../../queries/auth/sign_up';

export const SignUpModal: FC = () => {
  const navigate = useNavigate();

  const {
    register,
    handleSubmit,
    watch,
    setError,
    formState: { errors },
  } = useForm<SignUpInputs>();

  const { signUpMutation } = useSignUp(setError);

  const onSubmit: SubmitHandler<SignUpInputs> = (data) =>
    signUpMutation.mutate(data);

  return (
    <Modal modalClassName={styles.form} onOverlayClick={() => navigate('/')}>
      <div className={styles.title}>Sign Up</div>
      <Input
        placeholder="Email"
        type="email"
        {...register('email', { required: 'please enter a valid email' })}
      />
      <Input
        placeholder="Username"
        {...register('username', { required: 'please enter a username' })}
      />
      <Input
        placeholder="Password"
        type="password"
        {...register('password', { required: 'please enter a valid password' })}
      />
      <Input
        placeholder="Confirm Password"
        type="password"
        {...register('confirmPassword', {
          required: 'please confirm your password',
          validate: (value) =>
            value === watch('password') || 'passwords do not match',
        })}
      />
      {errors.root && (
        <span className={styles.error_text}>{errors.root.message}</span>
      )}
      <Button variant="black" onClick={handleSubmit(onSubmit)}>
        Sign Up
      </Button>
      <div className={styles.redirection_text}>
        Already have an account? <Link to="/sign_in">Sign In</Link>
      </div>
    </Modal>
  );
};
