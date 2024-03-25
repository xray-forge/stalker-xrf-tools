import { CircularProgress, Divider, Grid, Typography } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement } from "react";

import { ExportsViewerDeclaration } from "@/applications/exports_viewer/components/viewer/declarations/ExportsViewerDeclaration";
import { ExportsManager } from "@/applications/exports_viewer/store/exports";

export function ExportsViewerDialogs({
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
      <Typography variant={"h5"}>Dialogs</Typography>
      <Divider sx={{ margin: "16px 0" }} />

      <Grid direction={"column"} flexGrow={1} gap={1} flexWrap={"nowrap"} sx={{ overflowY: "auto" }} container>
        {declarations.dialogs.map((descriptor) => (
          <ExportsViewerDeclaration key={descriptor.name} descriptor={descriptor} />
        ))}
      </Grid>
    </Grid>
  );
}
