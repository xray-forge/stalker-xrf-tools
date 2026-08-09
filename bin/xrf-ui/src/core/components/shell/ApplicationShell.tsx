import { Box } from "@mui/material";
import { ReactElement, ReactNode } from "react";

import { ApplicationRail } from "@/core/components/shell/ApplicationRail";
import { ApplicationStatusBar } from "@/core/components/shell/ApplicationStatusBar";
import { EditorStatusProvider } from "@/core/components/shell/EditorStatusContext";

export interface IApplicationShellProps {
  children: ReactNode;
}

/**
 * Window chrome that outlives every route: tool rail on the left, status bar along the bottom.
 *
 * Routes render into the middle only. Nothing inside can take the window over, which is what separates
 * a desktop tool from a stack of full screen pages.
 */
export function ApplicationShell({ children }: IApplicationShellProps): ReactElement {
  return (
    <EditorStatusProvider>
      <Box sx={{ display: "flex", flexDirection: "column", width: "100%", height: "100%", flexWrap: "nowrap" }}>
        <Box sx={{ display: "flex", flexGrow: 1, minHeight: 0, flexWrap: "nowrap" }}>
          <ApplicationRail />

          <Box sx={{ display: "flex", flexGrow: 1, minWidth: 0, minHeight: 0, overflow: "hidden" }}>{children}</Box>
        </Box>

        <ApplicationStatusBar />
      </Box>
    </EditorStatusProvider>
  );
}
