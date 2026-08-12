import { Box, Typography } from "@mui/material";
import { ReactElement } from "react";

import { useEditorStatusSegments } from "@/core/components/shell/EditorStatusContext";
import { LAYOUT } from "@/lib/theme/tokens";

/**
 * Bottom status strip.
 */
export function ApplicationStatusBar(): ReactElement {
  const segments: Array<string> = useEditorStatusSegments();

  return (
    <Box
      sx={{
        display: "flex",
        alignItems: "center",
        gap: 1.5,
        height: LAYOUT.statusBarHeight,
        minHeight: LAYOUT.statusBarHeight,
        paddingX: 1.5,
        borderTop: 1,
        borderColor: "divider",
        backgroundColor: "background.paper",
      }}
    >
      {segments.length ? (
        segments.map((segment: string, index: number) => (
          <Typography key={segment + index} variant={"caption"} noWrap sx={{ color: "text.secondary" }}>
            {segment}
          </Typography>
        ))
      ) : (
        <Typography variant={"caption"} sx={{ color: "text.secondary", opacity: 0.7 }}>
          Ready
        </Typography>
      )}
    </Box>
  );
}
