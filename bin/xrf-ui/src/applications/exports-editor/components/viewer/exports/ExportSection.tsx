import { Box, Divider, Typography } from "@mui/material";
import { ReactElement, ReactNode } from "react";

import { BaseComponentProps } from "@/lib/dom/element-types";

export interface IExportSectionProps extends BaseComponentProps {
  isLast?: boolean;
  title: string;
  children: ReactNode;
}

export function ExportSection({ children, isLast = false, title }: IExportSectionProps): ReactElement {
  return (
    <Box component={"section"} sx={{ paddingBottom: isLast ? 0 : 2.5 }}>
      <Typography variant={"subtitle2"} sx={{ marginBottom: 1 }}>
        {title}
      </Typography>

      {children}

      {isLast ? null : <Divider sx={{ marginTop: 2.5 }} />}
    </Box>
  );
}
