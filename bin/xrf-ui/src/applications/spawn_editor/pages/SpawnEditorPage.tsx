import { CircularProgress, Grid } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { SpawnEditor } from "@/applications/spawn_editor/components/editor/SpawnEditor";
import { SpawnEditorOpenForm } from "@/applications/spawn_editor/components/SpawnEditorOpenForm";
import { SpawnFileManager } from "@/applications/spawn_editor/store/spawn";

export function SpawnEditorPage(): ReactElement {
  const { spawnFile, isReady } = useInjection(SpawnFileManager);

  if (isReady) {
    return spawnFile.value ? <SpawnEditor /> : <SpawnEditorOpenForm />;
  }

  return (
    <Grid container sx={{ width: "100%", height: "100%", justifyContent: "center", alignItems: "center" }}>
      <CircularProgress />
    </Grid>
  );
}
