import { FunctionComponent, useState } from "react";
import { Typography } from "../../components";

export const OnClick: FunctionComponent = (props) => {
  const [click, setClick] = useState(0);
  const handleClick = () => {
    setClick((_click) => _click + 1);
  };
  return (
    <>
      <button onClick={handleClick}>Increase Counter</button>
      <Typography variant="p">Current value: {click}</Typography>
    </>
  );
};
