import { FC, PropsWithChildren } from 'react';
import { useParams } from 'react-router-dom';
import { RealtimeProvider } from '../../contexts/realtime';

export const PlayPageRealtimeProvider: FC<PropsWithChildren> = ({
  children,
}) => {
  const matchId = useParams().match_id!;

  return <RealtimeProvider matchId={matchId}>{children}</RealtimeProvider>;
};
