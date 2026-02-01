import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useNavigate } from 'react-router-dom';

export const useLogout = () => {
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  const logoutMutation = useMutation({
    mutationFn: () =>
      fetch('/api/auth/logout', { method: 'POST', credentials: 'include' }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['auth_data'] });
      navigate('/');
    },
  });

  return { logoutMutation };
};
