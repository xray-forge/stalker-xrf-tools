import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { SpawnEditor } from "@/applications/spawn-editor/components/editor/SpawnEditor";
import { SpawnEditorOpenForm } from "@/applications/spawn-editor/components/SpawnEditorOpenForm";
import { SpawnFileService } from "@/applications/spawn-editor/store/spawn";
import { DelayedProgress } from "@/core/components/layout/DelayedProgress";

export function SpawnEditorPage(): ReactElement {
  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);

  if (!spawnFileService.isReady) {
    return <DelayedProgress />;
  }

  return spawnFileService.isOpen ? <SpawnEditor /> : <SpawnEditorOpenForm />;
}
