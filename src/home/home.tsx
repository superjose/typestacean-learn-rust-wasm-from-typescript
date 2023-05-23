import type { FunctionComponent } from "react";
import { GridMenu } from "../components/GridMenu";
import { routes } from "../utils/routes";
import { Typography } from "../components";

export const Home: FunctionComponent = () => {
  return (
    <>
      <Typography variant="h1">Menu</Typography>
      <GridMenu
        items={[
          {
            text: "Equivalencies",
            href: routes.equivalencies.index,
          },
        ]}
      />
    </>
  );
};
