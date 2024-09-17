import { CircularProgress, Grid } from "@mui/material";
import { useManager } from "dreamstate";

import { ExportsOpenForm } from "@/applications/exports_editor/components/ExportsOpenForm";
import { ExportsEditor } from "@/applications/exports_editor/components/viewer/ExportsEditor";
import { ExportsManager } from "@/applications/exports_editor/store/exports";

export function ExportsEditorPage({
  exportsContext: { isReady, declarations: { value: declarations } } = useManager(ExportsManager),
}) {
  if (isReady) {
    return declarations ? <ExportsEditor /> : <ExportsOpenForm />;
  }

  return (
    <Grid width={"100%"} height={"100%"} justifyContent={"center"} alignItems={"center"} container>
      <CircularProgress />
    </Grid>
  );
}
