import { Box } from "@mui/material";
import { ReactElement, ReactNode, useCallback, useEffect, useState } from "react";
import { useLocation } from "react-router-dom";

import { findApplicationTool } from "@/core/components/shell/application-tools";
import { ApplicationRail } from "@/core/components/shell/ApplicationRail";
import { ApplicationStatusBar } from "@/core/components/shell/ApplicationStatusBar";
import { ApplicationToolStripe } from "@/core/components/shell/ApplicationToolStripe";
import { IEditorTool, useEditorToolsRegistry } from "@/core/components/shell/EditorToolsContext";
import { Nullable } from "@/core/types/general";
import { getLocalStorageValue, setLocalStorageValue } from "@/lib/local-storage";

const TOOL_PANEL_WIDTH: number = 300;
const STORAGE_PREFIX: string = "xrf.tools.";

export interface IApplicationShellFrameProps {
  children: ReactNode;
}

/**
 * The window frame itself: rail on the left, tool stripe on the right, status bar along the bottom.
 *
 * Routes render into the middle only. Nothing inside can take the window over, which is what separates
 * a desktop tool from a stack of full screen pages. The stripe stays put even for editors that declare
 * no tools, for the same reason every route has a toolbar - a frame that changes shape as you move
 * between tools is harder to read than one that does not.
 */
export function ApplicationShellFrame({ children }: IApplicationShellFrameProps): ReactElement {
  const tools: Array<IEditorTool> = useEditorToolsRegistry();
  const { pathname } = useLocation();

  // Keyed per tool so the visuals panel choice does not leak into another editor.
  const storageKey: string = `${STORAGE_PREFIX}${findApplicationTool(pathname)?.path ?? "root"}`;

  const [activeToolId, setActiveToolId] = useState<Nullable<string>>(null);

  // Nothing stored means "not chosen yet", which resolves to the first tool: an editor that gains a
  // panel behaves as it did before the stripe existed, the panel is simply collapsible now. An empty
  // string is a deliberate collapse and stays collapsed.
  const resolvedToolId: Nullable<string> =
    activeToolId === null ? (tools[0]?.id ?? null) : tools.some((it) => it.id === activeToolId) ? activeToolId : null;

  const activeTool: Nullable<IEditorTool> = tools.find((it) => it.id === resolvedToolId) ?? null;

  const onToggleTool = useCallback(
    (id: string) => {
      const next: string = resolvedToolId === id ? "" : id;

      setActiveToolId(next);
      setLocalStorageValue(storageKey, next);
    },
    [resolvedToolId, storageKey]
  );

  useEffect(() => {
    setActiveToolId(getLocalStorageValue(storageKey));
  }, [storageKey]);

  return (
    <Box sx={{ display: "flex", flexDirection: "column", width: "100%", height: "100%", flexWrap: "nowrap" }}>
      <Box sx={{ display: "flex", flexGrow: 1, minHeight: 0, flexWrap: "nowrap" }}>
        <ApplicationRail />

        <Box sx={{ display: "flex", flexGrow: 1, minWidth: 0, minHeight: 0, overflow: "hidden" }}>{children}</Box>

        {activeTool ? (
          <Box
            sx={{
              display: "flex",
              flexDirection: "column",
              width: TOOL_PANEL_WIDTH,
              minWidth: TOOL_PANEL_WIDTH,
              minHeight: 0,
              overflowY: "auto",
              borderLeft: 1,
              borderColor: "divider",
              backgroundColor: "background.paper",
            }}
          >
            {activeTool.render()}
          </Box>
        ) : null}

        <ApplicationToolStripe tools={tools} activeToolId={resolvedToolId} onToggleTool={onToggleTool} />
      </Box>

      <ApplicationStatusBar />
    </Box>
  );
}
