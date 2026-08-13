import { useCallback, useEffect, useState } from "react";

import { IEditorPanel, TEditorPanelSide } from "@/core/shell/panel/context";
import { getLocalStorageValue, setLocalStorageValue } from "@/lib/local-storage";
import { PANEL } from "@/lib/theme/tokens";
import { Nullable } from "@/lib/types/general";

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
 */
export function usePanelSlot(side: TEditorPanelSide, panels: Array<IEditorPanel>, selectionScope: string): IPanelSlot {
  const storageKey: string = `xrf.panels.${side}.${selectionScope}`;

  const [activeId, setActiveId] = useState<Nullable<string>>(null);
  const [width, setWidth] = useState<number>(() => readWidth(side));

  const defaultPanelId: Nullable<string> = panels.find((panel) => panel.isOpenByDefault !== false)?.id ?? null;

  const resolvedPanelId: Nullable<string> =
    activeId === null ? defaultPanelId : panels.some((panel) => panel.id === activeId) ? activeId : null;

  const activePanel: Nullable<IEditorPanel> = panels.find((panel) => panel.id === resolvedPanelId) ?? null;

  const onResize = useCallback(
    (next: number) => {
      setWidth(next);
      setLocalStorageValue(`xrf.panels.${side}.width`, String(next));
    },
    [side]
  );

  const onTogglePanel = useCallback(
    (id: string) => {
      const next: string = resolvedPanelId === id ? "" : id;

      setActiveId(next);
      setLocalStorageValue(storageKey, next);
    },
    [resolvedPanelId, storageKey]
  );

  useEffect(() => {
    setActiveId(getLocalStorageValue(storageKey));
  }, [storageKey]);

  return { activePanel, activePanelId: resolvedPanelId, onResize, onTogglePanel, width };
}
