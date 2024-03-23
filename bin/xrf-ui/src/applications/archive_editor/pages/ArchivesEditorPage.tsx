import { CircularProgress, Grid } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement } from "react";

import { ArchivesEditorOpenForm } from "@/applications/archive_editor/components/ArchivesEditorOpenForm";
import { ArchivesEditor } from "@/applications/archive_editor/components/editor/ArchivesEditor";
import { ArchivesManager } from "@/applications/archive_editor/store/archives";

export function ArchivesEditorPage({
  archivesContext: { project, isReady } = useManager(ArchivesManager),
}): ReactElement {
  if (isReady) {
    return project.value ? <ArchivesEditor /> : <ArchivesEditorOpenForm />;
  }

  return (
    <Grid width={"100%"} height={"100%"} justifyContent={"center"} alignItems={"center"} container>
      <CircularProgress />
    </Grid>
  );
}
