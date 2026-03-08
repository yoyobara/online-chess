import { FC } from 'react';
import { Modal } from '../../../../components/Modal/Modal';
import styles from './PromotionModal.module.scss';
import { PieceColor, PieceType } from '../../../../types/piece';
import { getPieceSvg } from '../../../../utils/piece';
import { Button } from '../../../../components/Button/Button';

const AVAILABLE_PIECES: PieceType[] = ['Queen', 'Rook', 'Bishop', 'Knight'];

export interface PromotionModalProps {
  color: PieceColor;
  onClose: () => void;
  onPieceSelect: (piece: PieceType) => void;
}

export const PromotionModal: FC<PromotionModalProps> = ({
  color,
  onClose,
  onPieceSelect,
}: PromotionModalProps) => {
  return (
    <Modal onOverlayClick={onClose} modalClassName={styles.modal}>
      {AVAILABLE_PIECES.map((pieceType) => (
        <Button
          variant="white"
          onClick={() => {
            onPieceSelect(pieceType);
            onClose();
          }}
          key={pieceType}
        >
          <img
            alt={pieceType}
            src={getPieceSvg({ piece_type: pieceType, piece_color: color })}
          />
        </Button>
      ))}
    </Modal>
  );
};
