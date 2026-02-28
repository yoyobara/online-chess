import { File, Rank, Square, SquareColor } from '../types/square';

export const getSquareName = (index: number): Square => {
  const [row, column] = [Math.floor(index / 8), index % 8];

  const rank = (row + 1).toString() as File;
  const file = 'ABCDEFGH'.at(column) as Rank;

  return `${file}${rank}`;
};

export const getSquareColor = (index: number): SquareColor => {
  const [row, column] = [Math.floor(index / 8), index % 8];

  return (row + column) % 2 === 0 ? 'Dark' : 'Light';
};
