import { CircularProgress } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement } from "react";

import { SpawnEditor } from "@/applications/spawn_editor/components/editor/SpawnEditor";
import { SpawnSelectionForm } from "@/applications/spawn_editor/components/SpawnSelectionForm";
import { SpawnFileManager } from "@/applications/spawn_editor/store/spawn";

export function SpawnEditorPage({ spawnContext: { spawnFile, isReady } = useManager(SpawnFileManager) }): ReactElement {
  if (isReady) {
    return spawnFile.value ? <SpawnEditor /> : <SpawnSelectionForm />;
  }

  return <CircularProgress />;
}
