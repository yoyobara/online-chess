import { FC } from 'react';
import styles from './PlayPage.module.scss';
import { Button } from '../../components/Button/Button';
import { Paper } from '../../components/Paper/Paper';
import { PlayerPaper } from './player_paper/PlayerPaper';
import { Chessboard } from './chessboard/Chessboard';
import { useRequiredAuth } from '../../contexts/auth';
import { GameData } from '../../types/game_state';
import { useUserData } from '../../queries/user';

export interface PlayPageProps {
  game: GameData;
}

export const PlayPage: FC<PlayPageProps> = ({ game }) => {
  const me = useRequiredAuth();
  const opponent = useUserData(game.opponentId);

  return (
    <div className={styles.play_page}>
      <div className={styles.board_container}>
        <Chessboard board={game.currentBoard} myColor={game.myColor} />
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
        playerName={opponent?.username ?? null}
        playerRating={opponent?.rank ?? null}
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
