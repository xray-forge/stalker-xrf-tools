import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { SpawnEditorAlifeObjectsTable } from "@/applications/spawn/components/editor/chunks/alife/SpawnEditorAlifeObjectsTable";
import { SpawnChunkView } from "@/applications/spawn/components/editor/chunks/SpawnChunkView";
import { SpawnALifeSpawnsChunk } from "@/core/bindings/xrf-db";
import { SpawnFileService } from "@/core/spawn-file/services";

export function SpawnEditorAlife(): ReactElement {
  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);

  return (
    <SpawnChunkView<SpawnALifeSpawnsChunk>
      chunk={spawnFileService.alifeSpawn}
      render={(chunk: SpawnALifeSpawnsChunk) => <SpawnEditorAlifeObjectsTable objects={chunk.objects} />}
      onLoad={spawnFileService.loadAlifeSpawn}
    />
  );
}
