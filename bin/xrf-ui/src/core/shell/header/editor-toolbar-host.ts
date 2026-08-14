import { createContext, useContext } from "react";

import { Nullable } from "@/lib/types/general";

/**
 * The element the active application's toolbar is rendered into.
 */
export const EditorToolbarHostContext = createContext<Nullable<HTMLElement>>(null);

/**
 * Resolves the host for the active editor toolbar.
 *
 * @returns The toolbar host, or `null` before the frame slot mounts.
 */
export function useEditorToolbarHost(): Nullable<HTMLElement> {
  return useContext(EditorToolbarHostContext);
}
