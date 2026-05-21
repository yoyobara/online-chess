import { MatchResult } from '../types/match';
import { PieceColor } from '../types/piece';

export type PlayerStatus = 'win' | 'draw' | 'lose';

export const determinePlayerStatus = (
  myColor: PieceColor,
  res: MatchResult
): PlayerStatus => {
  switch (res.type) {
    case 'Draw':
      return 'draw';

    case 'Win':
      return res.data === myColor ? 'win' : 'lose';
  }
};
