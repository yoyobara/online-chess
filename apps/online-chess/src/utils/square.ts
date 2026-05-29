import { Square, SquareColor } from '../types/square';

export const getSquareName = (index: number): Square => {
  const [row, column] = [Math.floor(index / 8), index % 8];

  const rank = (row + 1).toString();
  const file = 'ABCDEFGH'.at(column);

  return `${file}${rank}` as Square;
};

export const getSquareColor = (index: number): SquareColor => {
  const [row, column] = [Math.floor(index / 8), index % 8];

  return (row + column) % 2 === 0 ? 'Dark' : 'Light';
};

export const getSquareIndex = (square: string): number => {
  const file = square[0].toUpperCase();
  const rank = Number(square[1]);

  const column = 'ABCDEFGH'.indexOf(file);
  const row = rank - 1;

  return row * 8 + column;
};
