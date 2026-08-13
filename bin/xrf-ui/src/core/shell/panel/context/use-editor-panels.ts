import { DependencyList, useContext, useEffect, useMemo } from "react";

import { useCurrentApplication } from "@/core/routing/current-application.context";
import { IEditorPanel } from "@/core/shell/panel/context/editor-panel";
import {
  EditorPanelsContext,
  EditorPanelsSetterContext,
  EMPTY_PANELS_STATE,
  IEditorPanelsState,
  NO_PANELS,
  TEditorPanelsStateSetter,
} from "@/core/shell/panel/context/EditorPanelsContext";

/** Only what the application on screen published. Anything left over from the last one is not rendered. */
export function useEditorPanelsRegistry(): ReadonlyArray<IEditorPanel> {
  const state: IEditorPanelsState = useContext(EditorPanelsContext);
  const owner: string = useCurrentApplication()?.path ?? "root";

  return state.owner === owner ? state.panels : NO_PANELS;
}

/**
 * Publish the panels this application offers, for as long as it is mounted.
 *
 * Dependencies follow `useMemo` semantics and decide when the factory publishes new render closures.
 * ESLint checks them at each call site, so captured values cannot change behind a stale panel array.
 */
export function useEditorPanels(createPanels: () => Array<IEditorPanel>, dependencies: DependencyList): void {
  const setState: TEditorPanelsStateSetter = useContext(EditorPanelsSetterContext);
  const owner: string = useCurrentApplication()?.path ?? "root";

  // Call sites are checked as dependency-aware hooks by ESLint.
  // eslint-disable-next-line react-hooks/exhaustive-deps
  const panels: Array<IEditorPanel> = useMemo(createPanels, dependencies);

  useEffect(() => {
    setState({ owner, panels });

    // Clears only what this application put there. Without the guard, an unmount that lands after the
    // next application has published would wipe the panels it just registered.
    return () => setState((previous) => (previous.owner === owner ? EMPTY_PANELS_STATE : previous));
  }, [owner, panels, setState]);
}
