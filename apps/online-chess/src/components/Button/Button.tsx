import { FC, MouseEventHandler, PropsWithChildren } from 'react';
import styles from './Button.module.scss';

interface ButtonProps extends PropsWithChildren {
  variant: 'black' | 'gray' | 'white' | 'red';
  onClick?: MouseEventHandler<HTMLButtonElement>;
  className?: string;
}

export const Button: FC<ButtonProps> = ({
  children,
  variant,
  onClick,
  className = '',
}) => {
  return (
    <button
      className={`${styles.button} ${styles[variant]} ${className}`}
      onClick={onClick}
    >
      {children}
    </button>
  );
};
