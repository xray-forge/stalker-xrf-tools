import { CircularProgress, Grid } from "@mui/material";
import { useInjection } from "@wirestate/react";

import { ExportsOpenForm } from "@/applications/exports-editor/components/ExportsOpenForm";
import { ExportsEditor } from "@/applications/exports-editor/components/viewer/ExportsEditor";
import { ExportsService } from "@/applications/exports-editor/store/exports";

export function ExportsEditorPage() {
  const exportsService: ExportsService = useInjection(ExportsService);

  if (exportsService.isReady) {
    return exportsService.project.value ? <ExportsEditor /> : <ExportsOpenForm />;
  }

  return (
    <Grid container sx={{ width: "100%", height: "100%", justifyContent: "center", alignItems: "center" }}>
      <CircularProgress />
    </Grid>
  );
}
