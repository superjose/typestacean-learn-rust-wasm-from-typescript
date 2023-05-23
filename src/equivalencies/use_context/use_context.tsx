import React, {
  useReducer,
  createContext,
  useContext,
  type FunctionComponent,
} from "react";

// You need to define the default state and reducer function somewhere
// const defaultMangaOptionsState = {...};
// const mangaOptionsReducer = (state, action) => {...};

const MangaOptionsContext = createContext{(});

type MangaOptionsContextType = {
  children: React.ReactNode;
};

export const MangaOptionsContextProvider: FunctionComponent<
  MangaOptionsContextType
> = ({ children }) => {
  const [state, dispatch] = useReducer(
    mangaOptionsReducer,
    defaultMangaOptionsState
  );

  return (
    <MangaOptionsContext.Provider value={{ state, dispatch }}>
      {children}
    </MangaOptionsContext.Provider>
  );
};

export function useMangaOptionsContext() {
  return useContext(MangaOptionsContext);
}
