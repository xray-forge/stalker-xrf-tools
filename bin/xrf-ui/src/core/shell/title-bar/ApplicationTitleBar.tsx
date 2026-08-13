import { Box } from "@mui/material";
import { ReactElement, Ref } from "react";

import { ApplicationTitleBarIcon } from "@/core/shell/title-bar/ApplicationTitleBarIcon";
import { WindowControls } from "@/core/shell/title-bar/WindowControls";
import { LAYOUT } from "@/core/theme/tokens";
import { BaseComponentProps } from "@/lib/dom/element-types";

export interface IApplicationTitleBarProps extends BaseComponentProps {
  /** Where the active application portals its toolbar. The frame owns the element. */
  toolbarRef?: Ref<HTMLElement>;
}

/**
 * The window's single top band: drawn caption and the active application's toolbar in one row.
 */
export function ApplicationTitleBar({
  "data-testid": dataTestId = "application-title-bar",
  id = "application-title-bar",
  className,
  toolbarRef,
}: IApplicationTitleBarProps): ReactElement {
  return (
    <Box
      data-testid={dataTestId}
      data-tauri-drag-region={"deep"}
      id={id}
      className={className}
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

      <Box ref={toolbarRef} sx={{ display: "flex", alignItems: "center", flexGrow: 1, minWidth: 0, height: "100%" }} />

      <WindowControls />
    </Box>
  );
}
