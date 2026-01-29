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

export const PlayPage: FC = () => {
  const auth = useRequiredAuth();
  const { lastJsonMessage } = useRealtime(useParams().match_id!);

  const [matchState, setMatchState] = useState<MatchState | null>(null);

  useEffect(() => {
    if (lastJsonMessage?.type === 'JoinResponse') {
      setMatchState(lastJsonMessage.data.initial_state);
    }
  }, [lastJsonMessage]);

  return (
    <div className={styles.play_page}>
      <div className={styles.board_container}>
        <Chessboard pieces={matchState?.board.state ?? []} />
      </div>
      <Paper className={styles.chat}></Paper>
      <Paper className={styles.history}></Paper>
      <PlayerPaper
        playerName={auth.username}
        playerRating={1000}
        variant="white"
        className={styles.player}
      />
      <PlayerPaper
        playerName="dummy1"
        playerRating={200}
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
