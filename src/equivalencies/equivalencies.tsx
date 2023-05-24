import type { FunctionComponent } from "react";
import { Outlet } from "react-router-dom";
import { GridMenu } from "../components/grid_menu";
import { routes } from "../utils/routes";

export type EquivalenciesProps = {};

export const Equivalencies: FunctionComponent<EquivalenciesProps> = () => {
  return (
    <>
      <GridMenu
        items={[
          {
            text: "Conditional Rendering",
            href: routes.equivalencies.conditionalRendering,
          },
          {
            text: "Logging",
            href: routes.equivalencies.logging,
          },
          {
            text: "onClick",
            href: routes.equivalencies.onClick,
          },
          {
            text: "useReducer",
            href: routes.equivalencies.useReducer,
          },
        ]}
      />
      <Outlet />
    </>
  );
};
