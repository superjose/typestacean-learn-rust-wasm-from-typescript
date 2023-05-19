import React, { useContext } from "react";
import IconButton from "path/to/IconButton";
import Icon from "path/to/Icon";
import { MangaContext, MangaAction } from "path/to/MangaContext";

export const OnClickComponent = () => {
  const state = useContext(MangaContext);
  const prevChapterDisabled = state.prevChapterDisabled;
  const goPrev = () => {
    state.dispatch(MangaAction.Prev);
  };

  return (
    <IconButton
      className="hidden sm:block"
      onClick={goPrev}
      icon={Icon.DoubleLeftArrow}
      disabled={prevChapterDisabled} // you need to define prevChapterDisabled somewhere
    />
  );
};
