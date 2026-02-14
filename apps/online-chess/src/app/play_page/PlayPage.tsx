import { Dispatch, FC, SetStateAction, useEffect } from 'react';
import styles from './PlayPage.module.scss';
import { Button } from '../../components/Button/Button';
import { Paper } from '../../components/Paper/Paper';
import { PlayerPaper } from './player_paper/PlayerPaper';
import { Chessboard } from './chessboard/Chessboard';
import { useRequiredAuth } from '../../contexts/auth';
import { useRealtime } from '../../contexts/realtime';
import { getSquareName } from '../../utils/square';
import { getPieceByIndex } from '../../utils/board';
import { MatchState } from '../../types/match';
import { UserData } from '../../types/user';
import { PieceColor } from '../../types/piece';

export interface PlayPageProps {
  matchState: MatchState;
  setMatchState: Dispatch<SetStateAction<MatchState | null>>;
  opponentData: UserData;
  myColor: PieceColor;
}

export const PlayPage: FC<PlayPageProps> = ({
  matchState,
  setMatchState,
  opponentData,
  myColor,
}) => {
  const me = useRequiredAuth();
  const { lastMessage, sendMessage } = useRealtime();

  useEffect(() => {
    if (!lastMessage) return;

    switch (lastMessage.type) {
      case 'NewState':
        setMatchState(lastMessage.data);
        break;

      case 'MoveResult':
        break;
    }
  }, [lastMessage, setMatchState]);

  const handleMove = (srcIndex: number, destIndex: number) => {
    if (srcIndex === destIndex) {
      return;
    }

    const src = getSquareName(srcIndex);
    const dest = getSquareName(destIndex);

    console.log(`${src} to ${dest}`);
    sendMessage({
      type: 'PlayerMove',
      data: {
        src_square: src,
        dest_square: dest,
        captured_piece:
          getPieceByIndex(matchState.board, destIndex)?.piece_type ?? null,
      },
    });
  };

  return (
    <div className={styles.play_page}>
      <div className={styles.board_container}>
        <Chessboard
          board={matchState.board}
          handleMove={handleMove}
          myColor={myColor}
        />
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
