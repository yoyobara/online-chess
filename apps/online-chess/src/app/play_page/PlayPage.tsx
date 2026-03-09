import { FC } from 'react';
import styles from './PlayPage.module.scss';
import { Button } from '../../components/Button/Button';
import { Paper } from '../../components/Paper/Paper';
import { PlayerPaper } from './player_paper/PlayerPaper';
import { Chessboard } from './chessboard/Chessboard';
import { useRequiredAuth } from '../../contexts/auth';
import { GameState } from '../../types/game_state';
import { useUserData } from '../../queries/user';
import { Move } from '../../types/move';
import { PromotionModal } from './chessboard/promotion_modal/PromotionModal';
import { PieceType } from '../../types/piece';
import { determinePlayerStatus } from '../../utils/match';

export interface PlayPageProps {
  gameState: GameState;
  setWaitingForMoveResponse: (optimsiticMove: Move) => void;
  setWaitingForPromotionChoice: (move: Move) => void;
  onPromotionModalClose: () => void;
  onPromotionModalSelect: (pieceType: PieceType) => void;
}

export const PlayPage: FC<PlayPageProps> = ({
  gameState,
  setWaitingForMoveResponse,
  setWaitingForPromotionChoice,
  onPromotionModalClose,
  onPromotionModalSelect,
}) => {
  const { game } = gameState;

  const me = useRequiredAuth();
  const opponent = useUserData(game.opponentId);

  const isMyTurn = game.moveCount % 2 === (game.myColor === 'White' ? 0 : 1);

  return (
    <div className={styles.play_page}>
      <div className={styles.board_container}>
        <Chessboard
          board={game.currentBoard}
          myColor={game.myColor}
          disableDrag={isMyTurn ? game.opponentColor : true}
          setWaitingForMoveResponse={setWaitingForMoveResponse}
          setWaitingForPromotionChoice={setWaitingForPromotionChoice}
          optimisticMove={
            gameState.type === 'WaitForMoveResponse'
              ? gameState.optimisticMove
              : undefined
          }
        />
      </div>
      <Paper className={styles.chat}></Paper>
      <Paper className={styles.history}></Paper>
      <PlayerPaper
        playerName={me.username}
        playerRating={me.rank}
        variant="white"
        status={
          gameState.type === 'Ended'
            ? determinePlayerStatus(game.myColor, gameState.result)
            : undefined
        }
        className={styles.player}
      />
      <PlayerPaper
        playerName={opponent?.username ?? null}
        playerRating={opponent?.rank ?? null}
        variant="purple"
        status={
          gameState.type === 'Ended'
            ? determinePlayerStatus(game.opponentColor, gameState.result)
            : undefined
        }
        className={styles.opponent}
      />
      <div className={styles.buttons}>
        <Button variant="red">Resign</Button>
        <Button variant="white">Offer Draw</Button>
      </div>
      {gameState.type === 'WaitForPromotionChoice' && (
        <PromotionModal
          color={game.myColor}
          onClose={onPromotionModalClose}
          onPieceSelect={onPromotionModalSelect}
        />
      )}
    </div>
  );
};
