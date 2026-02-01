import { FC, PropsWithChildren, createContext, useContext } from 'react';
import { useMe } from '../queries/auth/me';
import { AuthData } from '../types/auth/auth_data';

const authContext = createContext<AuthData | null>(null);

export const AuthProvider: FC<PropsWithChildren> = ({ children }) => {
  const { data } = useMe();

  if (data === undefined) {
    return null;
  }

  return <authContext.Provider value={data}>{children}</authContext.Provider>;
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
