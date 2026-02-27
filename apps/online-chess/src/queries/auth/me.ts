import { useQuery } from '@tanstack/react-query';
import { AuthData } from '../../types/auth/auth_data';

export const useMe = () => {
  const { data } = useQuery<AuthData | null>({
    queryKey: ['auth_data'],
    queryFn: () =>
      fetch('/api/user/me', { credentials: 'include' }).then((resp) =>
        resp.ok ? resp.json() : null
      ),
  });

  return { data };
};
