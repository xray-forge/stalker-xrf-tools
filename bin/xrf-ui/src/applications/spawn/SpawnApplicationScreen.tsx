import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { SpawnEditor } from "@/applications/spawn/components/editor/SpawnEditor";
import { SpawnEditorOpenForm } from "@/applications/spawn/components/SpawnEditorOpenForm";
import { DelayedProgress } from "@/core/components/layout/DelayedProgress";
import { SpawnFileService } from "@/lib/spawn-file";

/** Picker until a spawn file is open, editor once it is. */
export function SpawnApplicationScreen(): ReactElement {
  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);

  if (!spawnFileService.isReady) {
    return <DelayedProgress />;
  }

  return spawnFileService.isOpen ? <SpawnEditor /> : <SpawnEditorOpenForm />;
}
