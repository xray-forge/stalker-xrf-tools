import { createContext, Dispatch, ReactElement, ReactNode, SetStateAction, useState } from "react";

import { IEditorPanel } from "@/core/shell/panel/context/editor-panel";

/**
 * What is published, and which application published it.
 */
export interface IEditorPanelsState {
  readonly owner: string;
  readonly panels: ReadonlyArray<IEditorPanel>;
}

/** Stable, so a mismatched owner does not hand out a new array on every render. */
export const NO_PANELS: ReadonlyArray<IEditorPanel> = [];

export const EMPTY_PANELS_STATE: IEditorPanelsState = { owner: "", panels: NO_PANELS };

export type TEditorPanelsStateSetter = Dispatch<SetStateAction<IEditorPanelsState>>;

export const EditorPanelsContext = createContext<IEditorPanelsState>(EMPTY_PANELS_STATE);

/** Separate so publishing does not make the publisher rerender in response to its own state. */
export const EditorPanelsSetterContext = createContext<TEditorPanelsStateSetter>(() => {});

export function EditorPanelsProvider({ children }: { children: ReactNode }): ReactElement {
  const [state, setState] = useState<IEditorPanelsState>(EMPTY_PANELS_STATE);

  return (
    <EditorPanelsSetterContext.Provider value={setState}>
      <EditorPanelsContext.Provider value={state}>{children}</EditorPanelsContext.Provider>
    </EditorPanelsSetterContext.Provider>
  );
}
