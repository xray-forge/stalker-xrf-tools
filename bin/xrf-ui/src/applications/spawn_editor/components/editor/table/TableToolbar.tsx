import { Box } from "@mui/material";
import { GridToolbarColumnsButton, GridToolbarContainer, GridToolbarQuickFilter } from "@mui/x-data-grid";
import { ReactElement } from "react";

export function TableToolbar(): ReactElement {
  return (
    <GridToolbarContainer>
      <GridToolbarColumnsButton />
      <Box flexGrow={1} />
      <GridToolbarQuickFilter />
    </GridToolbarContainer>
  );
}
