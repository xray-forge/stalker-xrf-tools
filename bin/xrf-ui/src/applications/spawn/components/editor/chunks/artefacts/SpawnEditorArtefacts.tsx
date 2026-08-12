import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { SpawnEditorArtefactsNodesTable } from "@/applications/spawn/components/editor/chunks/artefacts/SpawnEditorArtefactsNodesTable";
import { SpawnChunkView } from "@/applications/spawn/components/editor/chunks/SpawnChunkView";
import { ISpawnFileArtefactSpawnsChunk , SpawnFileService } from "@/lib/spawn-file";

export function SpawnEditorArtefacts(): ReactElement {
  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);

  return (
    <SpawnChunkView<ISpawnFileArtefactSpawnsChunk>
      chunk={spawnFileService.artefactSpawn}
      render={(chunk: ISpawnFileArtefactSpawnsChunk) => <SpawnEditorArtefactsNodesTable nodes={chunk.nodes} />}
      onLoad={spawnFileService.loadArtefactSpawn}
    />
  );
}
