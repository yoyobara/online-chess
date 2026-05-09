import { SquareColor } from '../types/square';

export type Rank = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7;
export type File = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7;

export const getRank = (index: number): Rank => Math.floor(index / 8) as Rank;
export const getFile = (index: number): File => (index % 8) as File;

export const fromCoords = (rank: number, file: number): number => rank * 8 + file;

export const fromAlgebraic = (name: string): number => {
  const file = name.toLowerCase().charCodeAt(0) - 'a'.charCodeAt(0);
  const rank = parseInt(name[1], 10) - 1;
  return fromCoords(rank, file);
};

export const getSquareName = (index: number): string => {
  const rank = getRank(index);
  const file = getFile(index);

  const rankName = (rank + 1).toString();
  const fileName = 'abcdefgh'.at(file);

  return `${fileName}${rankName}`;
};

export const getSquareColor = (index: number): SquareColor => {
  const rank = getRank(index);
  const file = getFile(index);

  return (rank + file) % 2 === 0 ? 'Dark' : 'Light';
};
