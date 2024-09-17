import { CircularProgress, Divider, Grid, Typography } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement } from "react";

import { ExportsEditorDeclarationList } from "@/applications/exports_editor/components/viewer/declarations/ExportsEditorDeclarationList";
import { ExportsManager } from "@/applications/exports_editor/store/exports";

export function ExportsViewerEffects({
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
      <Typography variant={"h5"}>Effects ({declarations.effects.length})</Typography>
      <Divider sx={{ margin: "16px 0" }} />
      <ExportsEditorDeclarationList descriptors={declarations.effects} />
    </Grid>
  );
}
