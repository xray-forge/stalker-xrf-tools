import { CircularProgress, Grid } from "@mui/material";
import { useManager } from "dreamstate";

import { ExportsOpenForm } from "@/applications/exports_viewer/components/ExportsOpenForm";
import { ExportsViewer } from "@/applications/exports_viewer/components/viewer/ExportsViewer";
import { ExportsManager } from "@/applications/exports_viewer/store/exports";

export function ExportsViewerPage({
  exportsContext: { isReady, declarations: { value: declarations } } = useManager(ExportsManager),
}) {
  if (isReady) {
    return declarations ? <ExportsViewer /> : <ExportsOpenForm />;
  }

  return (
    <Grid width={"100%"} height={"100%"} justifyContent={"center"} alignItems={"center"} container>
      <CircularProgress />
    </Grid>
  );
}
