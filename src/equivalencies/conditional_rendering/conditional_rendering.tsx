import type { FunctionComponent } from "react";

export type ConditionalRenderingProps = {
  variant: "primary" | "secondary";
};

export const ConditionalRendering: FunctionComponent<
  ConditionalRenderingProps
> = (props) => {
  return (
    <>
      {props.variant === "primary" && <p>Primary</p>}
      {props.variant === "secondary" && <p>Secondary</p>}
    </>
  );
};
