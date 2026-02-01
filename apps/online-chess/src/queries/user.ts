import { useQuery } from '@tanstack/react-query';
import { UserData } from '../types/user';

export const useUserData = (userId: number | null) => {
  const { data: user } = useQuery<UserData>({
    queryKey: ['user', userId],
    queryFn: async () => {
      if (!userId) return null;

      const resp = await fetch(`/api/user/${userId}`, {
        credentials: 'include',
      });

      return resp.ok ? resp.json() : null;
    },

    enabled: !!userId,
  });

  return { user };
};
