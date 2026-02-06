import { FC, useEffect, useState } from 'react';
import styles from './PlayPage.module.scss';
import { Button } from '../../components/Button/Button';
import { Paper } from '../../components/Paper/Paper';
import { PlayerPaper } from './player_paper/PlayerPaper';
import { Chessboard } from './chessboard/Chessboard';
import { useRequiredAuth } from '../../contexts/auth';
import { MatchState } from '../../types/match';
import { useUserData } from '../../queries/user';
import { useRealtime } from '../../contexts/realtime';

export const PlayPage: FC = () => {
  const me = useRequiredAuth();

  const [matchState, setMatchState] = useState<MatchState | null>(null);
  const [opponentId, setOpponentId] = useState<number | null>(null);

  const { lastMessage, sendMessage } = useRealtime();
  const { user: opponentData } = useUserData(opponentId);

  useEffect(() => {
    if (!lastMessage) return;

    switch (lastMessage.type) {
      case 'JoinResponse':
        setMatchState(lastMessage.data.initial_state);
        setOpponentId(lastMessage.data.opponent_id);
        break;

      case 'MoveResult':
        if (lastMessage.data) {
          setMatchState(lastMessage.data);
        }
        break;

      case 'NewState':
        setMatchState(lastMessage.data);
        break;
    }
  }, [lastMessage]);

  const handleMove = (src: string, dest: string) => {
    console.log(`${src} to ${dest}`);

    sendMessage({
      type: 'PlayerMove',
      data: {
        src_square: src,
        dest_square: dest,
      },
    });
  };

  if (!matchState || !opponentData) {
    return null;
  }

  return (
    <div className={styles.play_page}>
      <div className={styles.board_container}>
        <Chessboard board={matchState.board} handleMove={handleMove} />
      </div>
      <Paper className={styles.chat}></Paper>
      <Paper className={styles.history}></Paper>
      <PlayerPaper
        playerName={me.username}
        playerRating={me.rank}
        variant="white"
        className={styles.player}
      />
      <PlayerPaper
        playerName={opponentData.username}
        playerRating={opponentData.rank}
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
