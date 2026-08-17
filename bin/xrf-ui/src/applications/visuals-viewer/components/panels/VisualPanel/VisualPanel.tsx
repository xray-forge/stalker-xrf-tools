import { Box, Typography } from "@mui/material";
import { ReactElement, ReactNode } from "react";

export interface IVisualPanelProps {
  /** The name this panel's stripe button carries. */
  title: string;
  children: ReactNode;
}

/**
 * One panel, titled with the name its stripe button carries.
 */
export function VisualPanel({ title, children }: IVisualPanelProps): ReactElement {
  return (
    <Box sx={{ display: "flex", flexDirection: "column", minHeight: 0 }}>
      <Box sx={{ paddingX: 2, paddingY: 1.5, borderBottom: 1, borderColor: "divider" }}>
        <Typography variant={"subtitle1"}>{title}</Typography>
      </Box>

      <Box sx={{ flexGrow: 1, minHeight: 0 }}>{children}</Box>
    </Box>
  );
}
