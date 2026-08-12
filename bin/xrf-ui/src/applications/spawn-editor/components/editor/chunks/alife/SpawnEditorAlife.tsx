import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { SpawnEditorAlifeObjectsTable } from "@/applications/spawn-editor/components/editor/chunks/alife/SpawnEditorAlifeObjectsTable";
import { SpawnChunkView } from "@/applications/spawn-editor/components/editor/chunks/SpawnChunkView";
import { SpawnFileService } from "@/applications/spawn-editor/store/spawn";
import { ISpawnFileAlifeSpawnsChunk } from "@/lib/spawn-file";

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
