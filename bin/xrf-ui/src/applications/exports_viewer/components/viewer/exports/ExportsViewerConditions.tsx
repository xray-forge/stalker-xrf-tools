import { CircularProgress, Divider, Grid, Typography } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement } from "react";

import { ExportsViewerDeclarationList } from "@/applications/exports_viewer/components/viewer/declarations/ExportsViewerDeclarationList";
import { ExportsManager } from "@/applications/exports_viewer/store/exports";

export function ExportsViewerConditions({
  exportsContext: { declarations: { isLoading, error, value: declarations } } = useManager(ExportsManager),
}): ReactElement {
  if (isLoading) {
    return (
      <Grid justifyContent={"center"} alignItems={"center"} width={"auto"} height={"100%"} flexGrow={1} container>
        <CircularProgress />
      </Grid>
    );
  }

  if (error || !declarations) {
    return (
      <Grid justifyContent={"center"} alignItems={"center"} width={"auto"} height={"100%"} flexGrow={1} container>
        {error ? String(error) : "No value."}
      </Grid>
    );
  }

  return (
    <Grid
      width={"auto"}
      height={"100%"}
      direction={"column"}
      flexWrap={"nowrap"}
      overflow={"auto"}
      p={2}
      flexGrow={1}
      container
    >
      <Typography variant={"h5"}>Conditions ({declarations.conditions.length})</Typography>
      <Divider sx={{ margin: "16px 0" }} />
      <ExportsViewerDeclarationList descriptors={declarations.conditions} />
    </Grid>
  );
}
