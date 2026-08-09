import { CircularProgress } from "@mui/material";
import { ReactElement } from "react";

import { CenteredColumn } from "@/core/components/layout/CenteredColumn";

/**
 * Suspense fallback shown while an editor's chunk is fetched.
 */
export function ApplicationLoader(): ReactElement {
  return (
    <CenteredColumn>
      <CircularProgress />
    </CenteredColumn>
  );
}
