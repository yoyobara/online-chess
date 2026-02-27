import { FC, PropsWithChildren } from 'react';
import { useAuth } from '../../contexts/auth';
import { Navigate } from 'react-router-dom';

export const LoggedInOnly: FC<PropsWithChildren> = ({ children }) => {
  const me = useAuth();

  return me ? children : <Navigate to="/" />;
};
