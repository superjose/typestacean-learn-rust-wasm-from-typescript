import type { FunctionComponent } from "react";

export type ButtonProps = {
  onClick(): void;
  children: string;
};

export const Button: FunctionComponent<ButtonProps> = (props) => {
  const { onClick, children } = props;
  return (
    <button className="px-4 py-2 bg-teal-500 rounded-lg" onClick={onClick}>
      {children}
    </button>
  );
};
