import { CircularProgress } from "@mui/material";
import { keyframes } from "@mui/system";
import { ReactElement } from "react";

import { CenteredColumn } from "@/core/components/layout/CenteredColumn";
import { BaseComponentProps } from "@/lib/dom/element-types";

const LOADER_KEYFRAMES = keyframes`
  from { visibility: hidden; }
  to { visibility: visible; }
`;

/** Centered progress indicator that stays hidden during fast operations. */
export function DelayedProgress({ "data-testid": dataTestId, id, className }: BaseComponentProps): ReactElement {
  return (
    <CenteredColumn
      data-testid={dataTestId}
      id={id}
      className={className}
      sx={{
        visibility: "hidden",
        animationName: `${LOADER_KEYFRAMES}`,
        animationDuration: "0s",
        animationDelay: "500ms",
        animationFillMode: "forwards",
      }}
    >
      <CircularProgress />
    </CenteredColumn>
  );
}
