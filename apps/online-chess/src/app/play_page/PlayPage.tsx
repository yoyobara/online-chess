import { FC, useEffect, useState } from 'react';
import styles from './PlayPage.module.scss';
import { Button } from '../../components/Button/Button';
import { Paper } from '../../components/Paper/Paper';
import { PlayerPaper } from './player_paper/PlayerPaper';
import { Chessboard } from './chessboard/Chessboard';
import { useRequiredAuth } from '../../contexts/auth';
import { useParams } from 'react-router-dom';
import { useRealtime } from '../../hooks/realtime';
import { MatchState } from '../../types/match';
import { useUserData } from '../../queries/user';

export const PlayPage: FC = () => {
  const me = useRequiredAuth();

  const [matchState, setMatchState] = useState<MatchState | null>(null);
  const [opponentId, setOpponentId] = useState<number | null>(null);

  const { lastJsonMessage } = useRealtime(useParams().match_id!);
  const { user: opponentData } = useUserData(opponentId);

  useEffect(() => {
    if (lastJsonMessage?.type === 'JoinResponse') {
      setMatchState(lastJsonMessage.data.initial_state);
      setOpponentId(lastJsonMessage.data.opponent_id);
    }
  }, [lastJsonMessage]);

  if (!matchState) {
    return null;
  }

  return (
    <div className={styles.play_page}>
      <div className={styles.board_container}>
        <Chessboard board={matchState.board} />
      </div>
      <Paper className={styles.chat}></Paper>
      <Paper className={styles.history}></Paper>
      <PlayerPaper
        playerName={me.username}
        playerRating={me.rank ?? -999}
        variant="white"
        className={styles.player}
      />
      <PlayerPaper
        playerName={opponentData?.username ?? '...'}
        playerRating={opponentData?.rank ?? -999}
        variant="purple"
        className={styles.opponent}
      />
      <div className={styles.buttons}>
        <Button variant="red">Resign</Button>
        <Button variant="white">Offer Draw</Button>
      </div>
    </div>
  );
};
