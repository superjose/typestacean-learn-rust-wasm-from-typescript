import type { FunctionComponent } from "react";
import { Typography } from "../../components";

type ChangedBy = "navigation" | "manually";

export const Logging: FunctionComponent = () => {
  const changedBy: ChangedBy = "navigation";
  console.log(`Hello TypeStacean 😁 ${changedBy}`);
  return (
    <>
      <Typography variant="h1">Check your console!</Typography>
    </>
  );
};
