import { CircularProgress, Grid } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { ArchivesEditorOpenForm } from "@/applications/archive_editor/components/ArchivesEditorOpenForm";
import { ArchivesEditor } from "@/applications/archive_editor/components/editor/ArchivesEditor";
import { ArchivesService } from "@/applications/archive_editor/store/archives";

export function ArchivesEditorPage(): ReactElement {
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
