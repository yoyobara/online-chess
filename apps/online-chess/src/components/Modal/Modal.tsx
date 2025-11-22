import { PropsWithChildren, FC, MouseEventHandler } from 'react';
import styles from './Modal.module.scss';

interface ModalProps extends PropsWithChildren {
  onOverlayClick?: MouseEventHandler<HTMLDivElement>;
  modalClassName?: string;
  overlayClassName?: string;
}

export const Modal: FC<ModalProps> = ({
  children,
  onOverlayClick,
  modalClassName = '',
  overlayClassName = '',
}) => {
  return (
    <>
      <div
        onClick={onOverlayClick}
        className={`${styles.overlay} ${overlayClassName}`}
      ></div>
      <div className={`${styles.modal} ${modalClassName}`}>{children}</div>
    </>
  );
};
