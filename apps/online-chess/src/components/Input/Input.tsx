import { FC, InputHTMLAttributes } from 'react';
import styles from './Input.module.scss';

type InputProps = InputHTMLAttributes<HTMLInputElement>;

export const Input: FC<InputProps> = ({ className, ...props }) => {
  return <input className={`${styles.input} ${className}`} {...props} />;
};
