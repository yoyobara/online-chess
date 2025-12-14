import { FC, useCallback, useEffect, useState } from 'react';
import styles from './PlayPage.module.scss';
import { Button } from '../../components/Button/Button';
import { Paper } from '../../components/Paper/Paper';
import { PlayerPaper } from './player_paper/PlayerPaper';
import { Chessboard } from './chessboard/Chessboard';
import { useRealtime } from '../../contexts/realtime';
import { MatchmakingModal } from '../matchmaking_modal/MatchmakingModal';
import { ServerMessage } from '../../types/messages';
import { useRequiredAuth } from '../../contexts/auth';
import { GameState } from './state';

export const PlayPage: FC = () => {
  const { lastServerMessage, sendMessage } = useRealtime();
  const auth = useRequiredAuth();

  const [gameState, setGameState] = useState<GameState>({
    type: 'beforeMatch',
  });

  const handleNewMessage = useCallback((msg: ServerMessage) => {
    switch (msg.type) {
      case 'WaitingForMatch':
        setGameState({ type: 'waitingForMatch' });
        break;

      case 'MatchFound':
        setGameState({ type: 'ongoingMatch', opponentName: msg.opponent_name });
        break;
    }
  }, []);

  // run on new ws message
  useEffect(() => {
    if (lastServerMessage === null) return;
    handleNewMessage(lastServerMessage);
  }, [lastServerMessage, handleNewMessage]);

  // run on mount
  useEffect(() => {
    sendMessage({ type: 'LookingForMatch' });
  }, [sendMessage]);

  return (
    <div className={styles.play_page}>
      <div className={styles.board_container}>
        <Chessboard
          pieces={[
            { color: 'white', type: 'bishop' },
            { color: 'black', type: 'queen' },
          ]}
        />
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
        playerName={
          gameState.type === 'ongoingMatch' ? gameState.opponentName : '???'
        }
        playerRating={200}
        variant="purple"
        className={styles.opponent}
      />
      <div className={styles.buttons}>
        <Button variant="red">Resign</Button>
        <Button variant="white">Offer Draw</Button>
      </div>
      {gameState.type === 'waitingForMatch' && <MatchmakingModal />}
    </div>
  );
};
