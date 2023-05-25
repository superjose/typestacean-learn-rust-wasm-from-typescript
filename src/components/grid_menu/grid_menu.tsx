import type { FunctionComponent } from "react";
import { GridItem } from "../grid_item/grid_item";
import { Link, useLocation } from "react-router-dom";

type Item = {
  text: string;
  href: string;
};

export type GridMenuProps = {
  items: Item[];
};

export const GridMenu: FunctionComponent<GridMenuProps> = (props) => {
  const location = useLocation();

  return (
    <div className="grid grid-cols-1 md:grid-cols-3">
      {props.items.map((item) => (
        <Link to={item.href}>
          <GridItem key={item.text} active={item.href === location.pathname}>
            {item.text}{" "}
          </GridItem>
        </Link>
      ))}
    </div>
  );
};
