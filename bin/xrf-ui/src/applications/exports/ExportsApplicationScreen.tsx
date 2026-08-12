import { CircularProgress, Grid } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { ExportsOpenForm } from "@/applications/exports/components/ExportsOpenForm";
import { ExportsEditor } from "@/applications/exports/components/viewer/ExportsEditor";
import { ExportsService } from "@/applications/exports/store/exports";

/** Picker until a project is open, viewer once it is. */
export function ExportsApplicationScreen(): ReactElement {
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
