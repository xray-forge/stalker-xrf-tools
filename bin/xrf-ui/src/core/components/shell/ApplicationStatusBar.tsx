import { Box, Typography } from "@mui/material";
import { ReactElement } from "react";
import { useLocation } from "react-router-dom";

import { APPLICATION_TOOLS, IApplicationTool } from "@/core/components/shell/applicationTools";
import { Optional } from "@/core/types/general";
import { LAYOUT } from "@/lib/theme/tokens";

/**
 * Bottom status strip.
 *
 * Currently reports only which tool is active. It exists now so editors have somewhere to put counts,
 * paths and progress instead of growing their own footers.
 */
export function ApplicationStatusBar(): ReactElement {
  const { pathname } = useLocation();

  const active: Optional<IApplicationTool> =
    APPLICATION_TOOLS.find((tool) => pathname.startsWith(tool.path)) ?? null;

  return (
    <Box
      sx={{
        display: "flex",
        alignItems: "center",
        gap: 1,
        height: LAYOUT.statusBarHeight,
        minHeight: LAYOUT.statusBarHeight,
        paddingX: 1.5,
        borderTop: 1,
        borderColor: "divider",
        backgroundColor: "background.paper",
      }}
    >
      <Typography variant={"caption"} sx={{ color: "text.secondary" }}>
        {active ? active.description : "XRF development tools"}
      </Typography>

      <Box sx={{ flexGrow: 1 }} />

      <Typography variant={"caption"} sx={{ color: "text.secondary" }}>
        {active ? active.label : "Home"}
      </Typography>
    </Box>
  );
}
