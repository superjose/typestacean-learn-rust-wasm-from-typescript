import type { FunctionComponent } from "react";
import { Typography } from "../../components";

export const Logging: FunctionComponent = () => {
  console.log("Hello TypeStacean 😁");
  return (
    <>
      <Typography variant="h1">Check your console</Typography>
    </>
  );
};
