import { Box, Typography } from "@mui/material";
import { ReactElement } from "react";

import { ApplicationTitleBarIcon } from "@/core/components/shell/title-bar/ApplicationTitleBarIcon";
import { WindowControls } from "@/core/components/shell/title-bar/WindowControls";
import { BaseComponentProps } from "@/lib/dom/element-types";
import { LAYOUT } from "@/lib/theme/tokens";

/**
 * Drawn window caption, replacing the one the system would have painted.
 */
export function ApplicationTitleBar({
  "data-testid": dataTestId = "application-title-bar",
  id = "application-title-bar",
}: BaseComponentProps): ReactElement {
  return (
    <Box
      data-testid={dataTestId}
      data-tauri-drag-region={"deep"}
      id={id}
      sx={{
        display: "flex",
        alignItems: "center",
        flexShrink: 0,
        height: LAYOUT.titleBarHeight,
        minHeight: LAYOUT.titleBarHeight,
        borderBottom: 1,
        borderColor: "divider",
        backgroundColor: "background.paper",
        userSelect: "none",
      }}
    >
      <ApplicationTitleBarIcon />

      <Typography>XRF</Typography>

      {/*
        Reserved for what the caption is expected to gain next - a menu bar, and the open document
        alongside it. Claiming the space now means adding either one moves nothing else on screen.
      */}
      <Box sx={{ display: "flex", alignItems: "center", flexGrow: 1, minWidth: 0 }} />
      <WindowControls />
    </Box>
  );
}
