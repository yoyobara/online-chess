import { MatchResult } from '../types/match';
import { PieceColor } from '../types/piece';

export const determinePlayerStatus = (
  myColor: PieceColor,
  res: MatchResult
) => {
  switch (res.type) {
    case 'Draw':
      return 'draw';

    case 'Win':
      return res.data === myColor ? 'win' : 'lose';
  }
};
