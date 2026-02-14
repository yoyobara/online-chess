export const getSquareName = (index: number) => {
  const [row, column] = [Math.floor(index / 8), index % 8];

  const rank = row + 1;
  const file = 'ABCDEFGH'.at(column);

  return `${file}${rank}`;
};
