import { FC } from 'react';
import styles from './PlayPage.module.scss';
import { Button } from '../../components/Button/Button';
import { Paper } from '../../components/Paper/Paper';
import { PlayerPaper } from './player_paper/PlayerPaper';
import { Chessboard } from './chessboard/Chessboard';
import { useRequiredAuth } from '../../contexts/auth';
import { GameState } from '../../types/game_state';
import { useUserData } from '../../queries/user';

export interface PlayPageProps {
  gameState: Exclude<GameState, { type: 'NotJoined' }>;
}

export const PlayPage: FC<PlayPageProps> = ({ gameState }) => {
  const { game } = gameState;

  const me = useRequiredAuth();
  const opponent = useUserData(game.opponentId);

  const opponentColor = game.myColor === 'White' ? 'Black' : 'White';
  const isMyTurn = game.moveCount % 2 === (game.myColor === 'White' ? 0 : 1);

  return (
    <div className={styles.play_page}>
      <div className={styles.board_container}>
        <Chessboard
          board={game.currentBoard}
          myColor={game.myColor}
          disableDrag={isMyTurn ? opponentColor : true}
        />
      </div>
      <Paper className={styles.chat}></Paper>
      <Paper className={styles.history}></Paper>
      <PlayerPaper
        playerName={me.username}
        playerRating={me.rank}
        variant="white"
        status={(gameState.type === 'Ended' && gameState.myStatus) || undefined}
        className={styles.player}
      />
      <PlayerPaper
        playerName={opponent?.username ?? null}
        playerRating={opponent?.rank ?? null}
        variant="purple"
        status={
          (gameState.type === 'Ended' && gameState.opponentStatus) || undefined
        }
        className={styles.opponent}
      />
      <div className={styles.buttons}>
        <Button variant="red">Resign</Button>
        <Button variant="white">Offer Draw</Button>
      </div>
    </div>
  );
};
