import type { FunctionComponent, ReactNode } from "react";
import cls from "clsx";

export type GridItemProps = {
  children: ReactNode;
  active?: boolean;
};

export const GridItem: FunctionComponent<GridItemProps> = (props) => {
  const { children, active } = props;
  return (
    <div
      className={cls(
        "bg-slate-400 hover:bg-slate-500 hover:cursor-pointer p-4",
        {
          "bg-purple-500": active,
        }
      )}
    >
      {children}
    </div>
  );
};
