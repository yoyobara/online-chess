import { Move } from '../types/move';

export const MoveToDTO = (mv: Move): Move => mv;

export const MoveFromDTO = (mvDto: Move): Move => mvDto;
