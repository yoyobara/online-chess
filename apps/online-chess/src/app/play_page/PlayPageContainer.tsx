import { FC, useEffect, useState } from 'react';
import { MatchState } from '../../types/match';
import { useRealtime } from '../../contexts/realtime';
import { useUserData } from '../../queries/user';
import { PlayPage } from './PlayPage';

export const PlayPageContainer: FC = () => {
  const [matchState, setMatchState] = useState<MatchState | null>(null);
  const [opponentId, setOpponentId] = useState<number | null>(null);

  const { lastMessage } = useRealtime();
  const { user: opponentData } = useUserData(opponentId);

  useEffect(() => {
    if (!lastMessage) return;

    switch (lastMessage.type) {
      case 'JoinResponse':
        setMatchState(lastMessage.data.initial_state);
        setOpponentId(lastMessage.data.opponent_id);
        break;
    }
  }, [lastMessage]);

  if (!matchState || !opponentData) {
    return null;
  }

  return (
    <PlayPage
      matchState={matchState}
      setMatchState={setMatchState}
      opponentData={opponentData}
    />
  );
};
