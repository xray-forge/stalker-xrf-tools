import { Box, Typography } from "@mui/material";
import { ReactElement } from "react";

export interface IVisualPanelEmptyProps {
  /** What is absent, and where it would come from once it is not. */
  label: string;
}

/** What a panel shows when no visual is open, or when the open one has nothing of this kind. */
export function VisualPanelEmpty({ label }: IVisualPanelEmptyProps): ReactElement {
  return (
    <Box sx={{ padding: 2 }}>
      <Typography variant={"body2"} sx={{ color: "text.secondary", lineHeight: 1.6 }}>
        {label}
      </Typography>
    </Box>
  );
}
