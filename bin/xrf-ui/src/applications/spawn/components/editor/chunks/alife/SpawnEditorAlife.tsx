import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { SpawnEditorAlifeObjectsTable } from "@/applications/spawn/components/editor/chunks/alife/SpawnEditorAlifeObjectsTable";
import { SpawnChunkView } from "@/applications/spawn/components/editor/chunks/SpawnChunkView";
import { ISpawnFileAlifeSpawnsChunk , SpawnFileService } from "@/lib/spawn-file";

export function SpawnEditorAlife(): ReactElement {
  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);

  return (
    <SpawnChunkView<ISpawnFileAlifeSpawnsChunk>
      chunk={spawnFileService.alifeSpawn}
      render={(chunk: ISpawnFileAlifeSpawnsChunk) => <SpawnEditorAlifeObjectsTable objects={chunk.objects} />}
      onLoad={spawnFileService.loadAlifeSpawn}
    />
  );
}
