import type { FunctionComponent } from "react";
import { Routes, Route, Outlet } from "react-router-dom";
import { ConditionalRendering } from "./conditional_rendering/conditional_rendering";
import { Logging } from "./logging/log";
import { GridMenu } from "../components/GridMenu";
import { routes } from "../utils/routes";
import { OnClick } from "./onclick/onclick";

export type EquivalenciesProps = {};

export const Equivalencies: FunctionComponent<EquivalenciesProps> = (props) => {
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
            text: "useContext",
            href: routes.equivalencies.useContext,
          },
        ]}
      />
      <Outlet />
    </>
  );
};
