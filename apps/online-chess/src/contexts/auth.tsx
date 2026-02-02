import { FC, PropsWithChildren, createContext, useContext } from 'react';
import { useMe } from '../queries/auth/me';
import { AuthData } from '../types/auth/auth_data';

const authContext = createContext<AuthData | null | undefined>(undefined);

export const AuthProvider: FC<PropsWithChildren> = ({ children }) => {
  const { data } = useMe();

  if (data === undefined) {
    return null;
  }

  return <authContext.Provider value={data}>{children}</authContext.Provider>;
};

export const useAuth = (): AuthData | null => {
  const context = useContext(authContext);

  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }

  return context;
};

export const useRequiredAuth = (): AuthData => {
  const auth = useAuth();

  if (auth === null) {
    throw new Error('useRequiredAuth must be used with an authenticated user');
  }

  return auth;
};
