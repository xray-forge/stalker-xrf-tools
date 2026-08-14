import { CircularProgress } from "@mui/material";
import { ReactElement } from "react";

import { CenteredColumn } from "@/core/ui/layout/CenteredColumn";
import { DELAYED_REVEAL_SX } from "@/core/ui/layout/delayed-reveal";
import { BaseComponentProps } from "@/lib/dom/element-types";

/** Centered progress indicator that stays hidden during fast operations. */
export function DelayedProgress({ "data-testid": dataTestId, id, className }: BaseComponentProps): ReactElement {
  return (
    <CenteredColumn data-testid={dataTestId} id={id} className={className} sx={DELAYED_REVEAL_SX}>
      <CircularProgress />
    </CenteredColumn>
  );
}
