import { Box, CircularProgress, Divider, Grid, Typography } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { ExportsEditorDeclarationList } from "@/applications/exports_editor/components/viewer/declarations/ExportsEditorDeclarationList";
import { ExportsService } from "@/applications/exports_editor/store/exports";

export function ExportsViewerConditions(): ReactElement {
  const exportsService: ExportsService = useInjection(ExportsService);

  if (exportsService.declarations.isLoading) {
    return (
      <Grid
        container
        sx={{ justifyContent: "center", alignItems: "center", width: "auto", height: "100%", flexGrow: 1 }}
      >
        <CircularProgress />
      </Grid>
    );
  }

  if (exportsService.declarations.error || !exportsService.declarations.value) {
    return (
      <Grid
        container
        sx={{ justifyContent: "center", alignItems: "center", width: "auto", height: "100%", flexGrow: 1 }}
      >
        {exportsService.declarations.error ? String(exportsService.declarations.error) : "No value."}
      </Grid>
    );
  }

  return (
    <Box
      sx={{
        display: "flex",
        width: "auto",
        height: "100%",
        flexDirection: "column",
        flexWrap: "nowrap",
        overflow: "auto",
        p: 2,
        flexGrow: 1,
      }}
    >
      <Typography variant={"h5"}>Conditions ({exportsService.declarations.value.conditions.length})</Typography>
      <Divider sx={{ margin: "16px 0" }} />
      <ExportsEditorDeclarationList descriptors={exportsService.declarations.value.conditions} />
    </Box>
  );
}
