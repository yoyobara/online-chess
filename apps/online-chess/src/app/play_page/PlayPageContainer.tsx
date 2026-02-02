import { useParams } from 'react-router-dom';
import { RealtimeProvider } from '../../contexts/realtime';
import { PlayPage } from './PlayPage';

export const PlayPageContainer = () => {
  const matchId = useParams().match_id!;

  return (
    <RealtimeProvider matchId={matchId}>
      <PlayPage />
    </RealtimeProvider>
  );
};
