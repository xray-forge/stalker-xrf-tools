import { createContext, useContext } from "react";

import { Nullable } from "@/lib/types/general";

/**
 * The element the active application's toolbar is rendered into.
 */
export const EditorToolbarHostContext = createContext<Nullable<HTMLElement>>(null);

/** Null until the frame's slot has mounted, and in any test that renders an editor without the frame. */
export function useEditorToolbarHost(): Nullable<HTMLElement> {
  return useContext(EditorToolbarHostContext);
}
