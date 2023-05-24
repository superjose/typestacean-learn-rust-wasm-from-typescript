import { useReducer, type FunctionComponent } from "react";
import { Typography } from "../../components";
import { Button } from "../../components/button";

type State = {
  value: number;
};

const defaultState: State = {
  value: 0,
};

type Action =
  | {
      type: "INCREASE";
    }
  | {
      type: "DECREASE";
    }
  | {
      type: "SET_VALUE";
      value: number;
    };

const reducer = (state: State, action: Action) => {
  switch (action.type) {
    case "DECREASE": {
      return {
        ...state,
        value: Math.max(0, state.value - 1),
      };
    }
    case "INCREASE": {
      return {
        ...state,
        value: state.value + 1,
      };
    }
    case "SET_VALUE": {
      return {
        ...state,
        value: action.value,
      };
    }
  }
};

export const UseReducer: FunctionComponent = () => {
  const [state, dispatch] = useReducer(reducer, defaultState);

  const increase = () => dispatch({ type: "INCREASE" });
  const decrease = () => dispatch({ type: "DECREASE" });
  const setValue = (value: number) => dispatch({ type: "SET_VALUE", value });

  return (
    <>
      <Typography variant="p">Value is {state.value}</Typography>
      <div className="flex justify-between max-w-sm">
        <Button onClick={increase}>Increase</Button>
        <Button onClick={decrease}>Decrease</Button>
        <Button onClick={() => setValue(0)}>Reset</Button>
      </div>
    </>
  );
};
