import { FC, HTMLAttributes } from 'react';
import styles from './Paper.module.scss';

type PaperProps = HTMLAttributes<HTMLDivElement>;

export const Paper: FC<PaperProps> = ({ children, className, ...props }) => {
  return (
    <div className={`${styles.paper} ${className}`} {...props}>
      {children}
    </div>
  );
};
