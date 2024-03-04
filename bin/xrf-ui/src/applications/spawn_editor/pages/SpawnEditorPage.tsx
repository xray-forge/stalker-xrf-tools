import { CircularProgress, Grid } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement } from "react";

import { SpawnEditor } from "@/applications/spawn_editor/components/editor/SpawnEditor";
import { SpawnEditorOpenForm } from "@/applications/spawn_editor/components/SpawnEditorOpenForm";
import { SpawnFileManager } from "@/applications/spawn_editor/store/spawn";

export function SpawnEditorPage({ spawnContext: { spawnFile, isReady } = useManager(SpawnFileManager) }): ReactElement {
  if (isReady) {
    return spawnFile.value ? <SpawnEditor /> : <SpawnEditorOpenForm />;
  }

  return (
    <Grid width={"100%"} height={"100%"} justifyContent={"center"} alignItems={"center"} container>
      <CircularProgress />
    </Grid>
  );
}
