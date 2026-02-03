export const getSquareName = (row: number, column: number) => {
  const rank = row + 1;
  const file = 'ABCDEFGH'.at(column);

  return `${file}${rank}`;
};
