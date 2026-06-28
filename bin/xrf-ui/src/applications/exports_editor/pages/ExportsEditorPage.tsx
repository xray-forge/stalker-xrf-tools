import { CircularProgress, Grid } from "@mui/material";
import { useInjection } from "@wirestate/react";

import { ExportsOpenForm } from "@/applications/exports_editor/components/ExportsOpenForm";
import { ExportsEditor } from "@/applications/exports_editor/components/viewer/ExportsEditor";
import { ExportsService } from "@/applications/exports_editor/store/exports";

export function ExportsEditorPage() {
  const exportsService: ExportsService = useInjection(ExportsService);

  if (exportsService.isReady) {
    return exportsService.declarations ? <ExportsEditor /> : <ExportsOpenForm />;
  }

  return (
    <Grid container sx={{ width: "100%", height: "100%", justifyContent: "center", alignItems: "center" }}>
      <CircularProgress />
    </Grid>
  );
}
