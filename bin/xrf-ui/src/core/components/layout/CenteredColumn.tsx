import { Box, SxProps, Theme } from "@mui/material";
import { ReactElement, ReactNode } from "react";

export interface ICenteredColumnProps {
  children: ReactNode;
  sx?: SxProps<Theme>;
}

/**
 * Full-size flex column that centers its children both axes.
 */
export function CenteredColumn({ children, sx }: ICenteredColumnProps): ReactElement {
  return (
    <Box
      sx={[
        {
          display: "flex",
          flexDirection: "column",
          justifyContent: "center",
          alignItems: "center",
          width: "100%",
          height: "100%",
          gap: 1,
        },
        ...(Array.isArray(sx) ? sx : [sx]),
      ]}
    >
      {children}
    </Box>
  );
}
