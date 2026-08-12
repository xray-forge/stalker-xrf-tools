import { useCallback, useEffect, useState } from "react";

import { IEditorPanel, TEditorPanelSide } from "@/core/components/shell/EditorPanelsContext";
import { GLOBAL_PANELS, isGlobalPanelId } from "@/core/components/shell/global-panels";
import { Nullable } from "@/core/types/general";
import { getLocalStorageValue, setLocalStorageValue } from "@/lib/local-storage";
import { PANEL } from "@/lib/theme/tokens";

export interface IPanelSlot {
  activePanel: Nullable<IEditorPanel>;
  activePanelId: Nullable<string>;
  width: number;
  onResize: (width: number) => void;
  onTogglePanel: (id: string) => void;
}

function readWidth(side: TEditorPanelSide): number {
  const stored: Nullable<string> = getLocalStorageValue(`xrf.panels.${side}.width`);
  const parsed: number = stored === null ? NaN : Number(stored);

  return Number.isFinite(parsed) ? Math.min(PANEL.maxWidth, Math.max(PANEL.minWidth, parsed)) : PANEL.defaultWidth;
}

/**
 * One side of the frame: which panel is showing there, and how wide it is.
 *
 * Both sides run through this, so left and right cannot drift into different rules about what an
 * unset choice means. Widths are per side and outlive the application; the open panel is remembered
 * per application, because the tree an editor wants open says nothing about the next one.
 */
export function usePanelSlot(
  side: TEditorPanelSide,
  panels: Array<IEditorPanel>,
  applicationPath: string
): IPanelSlot {
  const storageKey: string = `xrf.panels.${side}.${applicationPath}`;
  const globalStorageKey: string = `xrf.panels.${side}.global`;

  const [activeId, setActiveId] = useState<Nullable<string>>(null);
  const [globalId, setGlobalId] = useState<Nullable<string>>(() => getLocalStorageValue(globalStorageKey));
  const [width, setWidth] = useState<number>(() => readWidth(side));

  const defaultPanelId: Nullable<string> = panels.find((panel) => panel.isOpenByDefault !== false)?.id ?? null;

  // Nothing stored means "not chosen yet", which resolves to the first default-open panel. An empty
  // string is a deliberate collapse and stays collapsed.
  const resolvedPanelId: Nullable<string> =
    activeId === null ? defaultPanelId : panels.some((panel) => panel.id === activeId) ? activeId : null;

  const activeGlobalPanel: Nullable<IEditorPanel> =
    globalId && isGlobalPanelId(globalId) ? (GLOBAL_PANELS.find((panel) => panel.id === globalId) ?? null) : null;

  const activePanelId: Nullable<string> = activeGlobalPanel ? activeGlobalPanel.id : resolvedPanelId;

  const activePanel: Nullable<IEditorPanel> =
    activeGlobalPanel ?? panels.find((panel) => panel.id === activePanelId) ?? null;

  const onResize = useCallback(
    (next: number) => {
      setWidth(next);
      setLocalStorageValue(`xrf.panels.${side}.width`, String(next));
    },
    [side]
  );

  const onTogglePanel = useCallback(
    (id: string) => {
      if (isGlobalPanelId(id)) {
        const next: string = globalId === id ? "" : id;

        setGlobalId(next);
        setLocalStorageValue(globalStorageKey, next);

        return;
      }

      // Both claim the same slot, so picking an application panel has to release the global one -
      // otherwise the click reads as broken.
      if (activeGlobalPanel) {
        setGlobalId("");
        setLocalStorageValue(globalStorageKey, "");
      }

      // Collapsing by clicking the open panel again only applies when that panel is the one on screen.
      const next: string = !activeGlobalPanel && resolvedPanelId === id ? "" : id;

      setActiveId(next);
      setLocalStorageValue(storageKey, next);
    },
    [activeGlobalPanel, globalId, globalStorageKey, resolvedPanelId, storageKey]
  );

  useEffect(() => {
    setActiveId(getLocalStorageValue(storageKey));
  }, [storageKey]);

  return { activePanel, activePanelId, onResize, onTogglePanel, width };
}
