import { CircularProgress, Grid } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { ArchivesEditorOpenForm } from "@/applications/archives/components/ArchivesEditorOpenForm";
import { ArchivesEditor } from "@/applications/archives/components/editor/ArchivesEditor";
import { ArchivesService } from "@/applications/archives/store/archives";

/**
 * Picker until something is open, editor once it is.
 *
 * Separate from the application because a provider cannot consume what it provides.
 */
export function ArchivesApplicationScreen(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);

  if (archivesService.isReady) {
    return archivesService.project.value ? <ArchivesEditor /> : <ArchivesEditorOpenForm />;
  }

  return (
    <Grid container sx={{ width: "100%", height: "100%", justifyContent: "center", alignItems: "center" }}>
      <CircularProgress />
    </Grid>
  );
}
