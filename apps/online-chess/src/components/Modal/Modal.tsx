import { PropsWithChildren, FC } from 'react';
import styles from './Modal.module.scss';

interface ModalProps extends PropsWithChildren {
  modalClassName?: string;
  overlayClassName?: string;
}

export const Modal: FC<ModalProps> = ({
  children,
  modalClassName = '',
  overlayClassName = '',
}) => {
  return (
    <div className={`${styles.overlay} ${overlayClassName}`}>
      <div className={`${styles.modal} ${modalClassName}`}>{children}</div>
    </div>
  );
};
