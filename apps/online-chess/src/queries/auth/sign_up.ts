import { useMutation, useQueryClient } from '@tanstack/react-query';
import axios, { isAxiosError } from 'axios';
import { SignUpInputs } from '../../types/auth/sign_up';
import { useNavigate } from 'react-router-dom';
import { UseFormSetError } from 'react-hook-form';

export const useSignUp = (setError: UseFormSetError<SignUpInputs>) => {
  const navigate = useNavigate();
  const queryClient = useQueryClient();

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

  return { signUpMutation };
};
