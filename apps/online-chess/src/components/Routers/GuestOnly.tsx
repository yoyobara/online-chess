import { FC, PropsWithChildren } from 'react';
import { useAuth } from '../../contexts/auth';
import { Navigate } from 'react-router-dom';

export const GuestOnly: FC<PropsWithChildren> = ({ children }) => {
  const me = useAuth();

  return me ? <Navigate to="/home" /> : children;
};
