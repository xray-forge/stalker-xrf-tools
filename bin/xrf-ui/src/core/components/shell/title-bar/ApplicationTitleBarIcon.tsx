import { Box } from "@mui/material";
import { ReactElement } from "react";

/**
 * The application icon, where a window's own icon sits.
 */
export function ApplicationTitleBarIcon(): ReactElement {
  return (
    <Box
      component={"img"}
      src={"/icon.png"}
      alt={"XRF tools"}
      // Without this the browser's own image drag starts instead of the window moving.
      draggable={false}
      sx={{ width: 16, height: 16, marginX: 1, flexShrink: 0 }}
    />
  );
}
