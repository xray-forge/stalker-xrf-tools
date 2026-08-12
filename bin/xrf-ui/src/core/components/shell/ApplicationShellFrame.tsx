import { Box } from "@mui/material";
import { ReactElement, ReactNode, useCallback, useEffect, useState } from "react";
import { useLocation } from "react-router-dom";

import { ApplicationCrash } from "@/core/components/error/ApplicationCrash";
import { ErrorBoundary, IErrorBoundaryFallbackProps } from "@/core/components/error/ErrorBoundary";
import { findApplicationTool } from "@/core/components/shell/application-tools";
import { ApplicationRail } from "@/core/components/shell/ApplicationRail";
import { ApplicationStatusBar } from "@/core/components/shell/ApplicationStatusBar";
import { ApplicationToolStripe } from "@/core/components/shell/ApplicationToolStripe";
import { IEditorTool, useEditorToolsRegistry } from "@/core/components/shell/EditorToolsContext";
import { GLOBAL_TOOLS, isGlobalToolId } from "@/core/components/shell/global-tools";
import { ApplicationTitleBar } from "@/core/components/shell/title-bar/ApplicationTitleBar";
import { Nullable } from "@/core/types/general";
import { getLocalStorageValue, setLocalStorageValue } from "@/lib/local-storage";

const TOOL_PANEL_WIDTH: number = 300;
const STORAGE_PREFIX: string = "xrf.tools.";
const GLOBAL_STORAGE_KEY: string = "xrf.tools.global";

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
  const [globalToolId, setGlobalToolId] = useState<Nullable<string>>(() => getLocalStorageValue(GLOBAL_STORAGE_KEY));

  const defaultToolId: Nullable<string> = tools.find((tool) => tool.isOpenByDefault !== false)?.id ?? null;

  // Nothing stored means "not chosen yet", which resolves to the first default-open tool. An empty
  // string is a deliberate collapse and stays collapsed.
  const resolvedEditorToolId: Nullable<string> =
    activeToolId === null ? defaultToolId : tools.some((it) => it.id === activeToolId) ? activeToolId : null;

  const activeGlobalTool: Nullable<IEditorTool> =
    globalToolId && isGlobalToolId(globalToolId) ? (GLOBAL_TOOLS.find((it) => it.id === globalToolId) ?? null) : null;

  const resolvedToolId: Nullable<string> = activeGlobalTool ? activeGlobalTool.id : resolvedEditorToolId;

  const activeTool: Nullable<IEditorTool> = activeGlobalTool ?? tools.find((it) => it.id === resolvedToolId) ?? null;

  const onError = useCallback((props: IErrorBoundaryFallbackProps) => <ApplicationCrash {...props} />, []);

  const onToggleTool = useCallback(
    (id: string) => {
      if (isGlobalToolId(id)) {
        const next: string = globalToolId === id ? "" : id;

        setGlobalToolId(next);
        setLocalStorageValue(GLOBAL_STORAGE_KEY, next);

        return;
      }

      // Both claim the same slot, so picking an editor panel has to release the global one - otherwise
      // the click reads as broken.
      if (activeGlobalTool) {
        setGlobalToolId("");
        setLocalStorageValue(GLOBAL_STORAGE_KEY, "");
      }

      // Collapsing by clicking the open panel again only applies when that panel is the one on screen.
      const next: string = !activeGlobalTool && resolvedEditorToolId === id ? "" : id;

      setActiveToolId(next);
      setLocalStorageValue(storageKey, next);
    },
    [activeGlobalTool, globalToolId, resolvedEditorToolId, storageKey]
  );

  useEffect(() => {
    setActiveToolId(getLocalStorageValue(storageKey));
  }, [storageKey]);

  return (
    <Box sx={{ display: "flex", flexDirection: "column", width: "100%", height: "100%", flexWrap: "nowrap" }}>
      <ApplicationTitleBar />

      <Box sx={{ display: "flex", flexGrow: 1, minHeight: 0, flexWrap: "nowrap" }}>
        <ApplicationRail />

        <Box sx={{ display: "flex", flexGrow: 1, minWidth: 0, minHeight: 0, overflow: "hidden" }}>
          <ErrorBoundary resetKey={pathname} fallback={onError}>
            {children}
          </ErrorBoundary>
        </Box>

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
              backgroundColor: "background.default",
            }}
          >
            {activeTool.render()}
          </Box>
        ) : null}

        <ApplicationToolStripe
          tools={tools}
          globalTools={GLOBAL_TOOLS}
          activeToolId={resolvedToolId}
          onToggleTool={onToggleTool}
        />
      </Box>

      <ApplicationStatusBar />
    </Box>
  );
}
