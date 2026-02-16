import { SquareColor } from '../types/square';

export const getSquareName = (index: number): string => {
  const [row, column] = [Math.floor(index / 8), index % 8];

  const rank = row + 1;
  const file = 'ABCDEFGH'.at(column);

  return `${file}${rank}`;
};

export const getSquareColor = (index: number): SquareColor => {
  const [row, column] = [Math.floor(index / 8), index % 8];

  return (row + column) % 2 === 1 ? 'Dark' : 'Light';
};
