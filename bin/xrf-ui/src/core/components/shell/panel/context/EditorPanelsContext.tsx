import { createContext, ReactElement, ReactNode, useMemo, useState } from "react";

import { IEditorPanel } from "@/core/components/shell/panel/context/editor-panel";

/**
 * What is published, and which application published it.
 */
export interface IEditorPanelsState {
  owner: string;
  panels: Array<IEditorPanel>;
}

export interface IEditorPanelsContextValue {
  state: IEditorPanelsState;
  setState: (update: (previous: IEditorPanelsState) => IEditorPanelsState) => void;
}

/** Stable, so a mismatched owner does not hand out a new array on every render. */
export const NO_PANELS: Array<IEditorPanel> = [];

export const EMPTY_PANELS_STATE: IEditorPanelsState = { owner: "", panels: NO_PANELS };

export const EditorPanelsContext = createContext<IEditorPanelsContextValue>({
  setState: () => {},
  state: EMPTY_PANELS_STATE,
});

export function EditorPanelsProvider({ children }: { children: ReactNode }): ReactElement {
  const [state, setState] = useState<IEditorPanelsState>(EMPTY_PANELS_STATE);

  const value: IEditorPanelsContextValue = useMemo(() => ({ setState, state }), [state]);

  return <EditorPanelsContext.Provider value={value}>{children}</EditorPanelsContext.Provider>;
}
