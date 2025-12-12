import { FC } from 'react';
import styles from './auth_modal.module.scss';
import { Modal } from '../../components/Modal/Modal';
import { Button } from '../../components/Button/Button';
import { Input } from '../../components/Input/Input';
import { Link, useNavigate } from 'react-router-dom';
import { SubmitHandler, useForm } from 'react-hook-form';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import axios, { isAxiosError } from 'axios';

interface SignUpInputs {
  email: string;
  username: string;
  password: string;
  confirmPassword: string;
}

export const SignUpModal: FC = () => {
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  const {
    register,
    handleSubmit,
    watch,
    setError,
    formState: { errors },
  } = useForm<SignUpInputs>();

  const signUpMutation = useMutation({
    mutationFn: ({ confirmPassword, ...signUpRequest }: SignUpInputs) =>
      axios
        .post('/api/auth/register', signUpRequest, { withCredentials: true })
        .then((res) => res.data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['auth_data'] });
      navigate('/home');
    },
    onError: (e) => {
      if (isAxiosError(e)) {
        switch (e.status) {
          case 409:
            setError('root', {
              message: 'a user with this email or username already exists',
            });
            break;

          default:
            break;
        }
      }
    },
  });

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
