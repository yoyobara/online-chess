import { useQuery } from '@tanstack/react-query';
import { FC, PropsWithChildren, createContext, useContext } from 'react';

interface AuthData {
  id: number;
  username: string;
}

const authContext = createContext<AuthData | null>(null);

export const AuthProvider: FC<PropsWithChildren> = ({ children }) => {
  const { data, isLoading } = useQuery<AuthData | null>({
    queryKey: ['auth_data'],
    queryFn: () =>
      fetch('/api/auth/me', { credentials: 'include' }).then((resp) =>
        resp.ok ? resp.json() : null
      ),
  });

  if (isLoading) {
    return null;
  }

  return (
    <authContext.Provider value={data ?? null}>{children}</authContext.Provider>
  );
};

export const useAuth = (): AuthData | null => {
  const context = useContext(authContext);

  return context;
};

export const useRequiredAuth = (): AuthData => {
  const context = useContext(authContext);
  if (!context) {
    throw new Error(
      'useRequiredAuth must be used within AuthProvider with an authenticated user'
    );
  }

  return context;
};
