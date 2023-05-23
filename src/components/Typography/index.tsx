import type {
  CSSProperties,
  InputHTMLAttributes,
  PropsWithChildren,
} from "react";
import cls from "clsx";

export type TypographyProps = {
  className?: string;
  variant: "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "p" | "span";
  type?: "error";
  style?: CSSProperties;
};
type Props = PropsWithChildren<TypographyProps>;

function getAttributesFromProps(props: Props) {
  return {
    className: cls(
      {
        "text-4xl": props.variant === "h1",
        "text-red": props.type === "error",
      },

      props.className
    ),
    style: props.style,
  } as InputHTMLAttributes<HTMLHeadingElement>;
}

const H1 = (props: Props) => (
  <h1 {...getAttributesFromProps(props)}>{props.children}</h1>
);
const H2 = (props: Props) => (
  <h2 {...getAttributesFromProps(props)}>{props.children}</h2>
);
const H3 = (props: Props) => (
  <h3 {...getAttributesFromProps(props)}>{props.children}</h3>
);
const H4 = (props: Props) => (
  <h4 {...getAttributesFromProps(props)}>{props.children}</h4>
);
const H5 = (props: Props) => (
  <h5 {...getAttributesFromProps(props)}>{props.children}</h5>
);
const H6 = (props: Props) => (
  <h6 {...getAttributesFromProps(props)}>{props.children}</h6>
);
const P = (props: Props) => (
  <p {...getAttributesFromProps(props)}>{props.children}</p>
);

const S = (props: Props) => (
  <span {...getAttributesFromProps(props)}>{props.children}</span>
);

const typographyMap = {
  h1: H1,
  h2: H2,
  h3: H3,
  h4: H4,
  h5: H5,
  h6: H6,
  p: P,
  span: S,
};

export const Typography = (props: Props) => {
  const { variant } = props;
  return typographyMap[variant](props);
};
