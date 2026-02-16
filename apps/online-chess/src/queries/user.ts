import { useQuery } from '@tanstack/react-query';
import { UserData } from '../types/user';

export const useUserData = (userId: number) => {
  const { data: user } = useQuery<UserData>({
    queryKey: ['user', userId],
    queryFn: async () => {
      const resp = await fetch(`/api/user/${userId}`, {
        credentials: 'include',
      });

      if (!resp.ok) {
        throw Error('bad user query');
      }

      return resp.json();
    },
  });

  return user;
};
