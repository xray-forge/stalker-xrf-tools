import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { SpawnEditor } from "@/applications/spawn/components/editor/SpawnEditor";
import { SpawnEditorOpenForm } from "@/applications/spawn/components/SpawnEditorOpenForm";
import { SpawnFileService } from "@/core/spawn/services";
import { DelayedProgress } from "@/core/ui/layout/DelayedProgress";

/** Picker until a spawn file is open, editor once it is. */
export function SpawnApplication(): ReactElement {
  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);

  if (!spawnFileService.isReady) {
    return <DelayedProgress />;
  }

  return spawnFileService.isOpen ? <SpawnEditor /> : <SpawnEditorOpenForm />;
}
