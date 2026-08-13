import { Box } from "@mui/material";
import { ReactElement, ReactNode } from "react";

import { BaseComponentProps } from "@/lib/dom/element-types";

export interface ICenteredColumnProps extends BaseComponentProps {
  children: ReactNode;
}

/**
 * Full-size flex column that centers its children both axes.
 */
export function CenteredColumn({
  "data-testid": dataTestId,
  id,
  className,
  children,
  sx,
}: ICenteredColumnProps): ReactElement {
  return (
    <Box
      data-testid={dataTestId}
      id={id}
      className={className}
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
        ...(sx === undefined ? [] : Array.isArray(sx) ? sx : [sx]),
      ]}
    >
      {children}
    </Box>
  );
}
