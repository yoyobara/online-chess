import { Move, Move } from '../types/move';
import { getSquareIndex, getSquareName } from './square';

export const MoveToDTO = (mv: Move): Move => ({
  dest_square: getSquareName(mv.destIndex),
  src_square: getSquareName(mv.srcIndex),
  move_type: mv.moveType,
  promotion: mv.promotion,
});

export const MoveFromDTO = (mvDto: Move): Move => ({
  destIndex: getSquareIndex(mvDto.dest_square),
  srcIndex: getSquareIndex(mvDto.src_square),
  moveType: mvDto.move_type,
  promotion: mvDto.promotion,
});
