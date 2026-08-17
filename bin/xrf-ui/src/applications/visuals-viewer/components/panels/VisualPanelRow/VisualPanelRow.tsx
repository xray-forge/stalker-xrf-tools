import { Box, Typography } from "@mui/material";
import { ReactElement, ReactNode } from "react";

export interface IVisualPanelRowProps {
  label: string;
  value: ReactNode;
}

/**
 * One label and value pair.
 *
 * The value is allowed to wrap onto its own line, because a texture path or a source object path is longer than the
 * panel is wide and truncating it would hide the part that identifies it.
 */
export function VisualPanelRow({ label, value }: IVisualPanelRowProps): ReactElement {
  return (
    <Box
      sx={{
        display: "flex",
        justifyContent: "space-between",
        alignItems: "baseline",
        gap: 2,
        paddingY: 0.4,
        lineHeight: 1.6,
      }}
    >
      <Typography variant={"body2"} sx={{ color: "text.secondary", flexShrink: 0 }}>
        {label}
      </Typography>

      <Typography variant={"body2"} sx={{ textAlign: "right", wordBreak: "break-all" }}>
        {value}
      </Typography>
    </Box>
  );
}
