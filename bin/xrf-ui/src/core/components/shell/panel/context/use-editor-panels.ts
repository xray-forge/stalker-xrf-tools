import { useCallback, useContext, useEffect, useRef } from "react";
import { useLocation } from "react-router-dom";

import { IEditorPanel } from "@/core/components/shell/panel/context/editor-panel";
import {
  EditorPanelsContext,
  EMPTY_PANELS_STATE,
  IEditorPanelsContextValue,
  NO_PANELS,
} from "@/core/components/shell/panel/context/EditorPanelsContext";
import { findApplication } from "@/core/router/applications";

/**
 * The application a panel belongs to, derived the way the frame derives its container scope.
 *
 * An application's inner routes all resolve to its own path, so the owner is stable while you move
 * about inside one.
 */
function useCurrentPanelOwner(): string {
  const { pathname } = useLocation();

  return findApplication(pathname)?.path ?? "root";
}

/** Only what the application on screen published. Anything left over from the last one is not rendered. */
export function useEditorPanelsRegistry(): Array<IEditorPanel> {
  const { state }: IEditorPanelsContextValue = useContext(EditorPanelsContext);
  const owner: string = useCurrentPanelOwner();

  return state.owner === owner ? state.panels : NO_PANELS;
}

/**
 * Publish the panels this application offers, for as long as it is mounted.
 *
 * Panels are compared by id, label and side rather than by array identity, so callers can declare them
 * inline without memoising.
 */
export function useEditorPanels(panels: Array<IEditorPanel>): void {
  const { setState }: IEditorPanelsContextValue = useContext(EditorPanelsContext);

  const owner: string = useCurrentPanelOwner();

  const latest = useRef<Array<IEditorPanel>>(panels);
  const key: string = panels
    .map((panel) => `${panel.id}:${panel.label}:${panel.side ?? "right"}:${panel.isOpenByDefault !== false}`)
    .join("|");

  latest.current = panels;

  const publish = useCallback(() => {
    setState(() => ({ owner, panels: latest.current }));

    // Clears only what this application put there. Without the guard, an unmount that lands after the
    // next application has published would wipe the panels it just registered.
    return () => setState((previous) => (previous.owner === owner ? EMPTY_PANELS_STATE : previous));
  }, [owner, setState]);

  useEffect(publish, [key, publish]);
}
