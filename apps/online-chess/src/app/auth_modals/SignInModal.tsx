import { FC } from 'react';
import styles from './auth_modal.module.scss';
import { Modal } from '../../components/Modal/Modal';
import { Button } from '../../components/Button/Button';
import { Input } from '../../components/Input/Input';
import { Link, useNavigate } from 'react-router-dom';
import { SubmitHandler, useForm } from 'react-hook-form';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import axios, { isAxiosError } from 'axios';

interface SignInInputs {
  email: string;
  password: string;
}

export const SignInModal: FC = () => {
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  const {
    register,
    handleSubmit,
    setError,
    formState: { errors },
  } = useForm<SignInInputs>();

  const loginMutation = useMutation({
    mutationFn: (loginRequest: SignInInputs) =>
      axios
        .post('/api/auth/login', loginRequest, { withCredentials: true })
        .then((res) => res.data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['auth_data'] });
      navigate('/home');
    },
    onError: (e) => {
      if (isAxiosError(e)) {
        switch (e.status) {
          case 401:
            setError('password', {
              message: 'the password does not match the email',
            });
            break;

          case 404:
            setError('email', {
              message: 'a user with that email could not be found',
            });
            break;

          default:
            break;
        }
      }
    },
  });

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
