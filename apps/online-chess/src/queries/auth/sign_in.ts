import { useMutation, useQueryClient } from '@tanstack/react-query';
import { SignInInputs } from '../../types/auth/sign_in';
import { useNavigate } from 'react-router-dom';
import axios, { isAxiosError } from 'axios';
import { UseFormSetError } from 'react-hook-form';

export const useSignIn = (setError: UseFormSetError<SignInInputs>) => {
  const queryClient = useQueryClient();
  const navigate = useNavigate();

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

  return { loginMutation };
};
