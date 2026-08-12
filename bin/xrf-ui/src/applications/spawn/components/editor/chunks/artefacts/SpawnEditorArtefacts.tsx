import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { SpawnEditorArtefactsNodesTable } from "@/applications/spawn/components/editor/chunks/artefacts/SpawnEditorArtefactsNodesTable";
import { SpawnChunkView } from "@/applications/spawn/components/editor/chunks/SpawnChunkView";
import { SpawnArtefactSpawnsChunk } from "@/lib/bindings/xray-db";
import { SpawnFileService } from "@/lib/spawn-file";

export function SpawnEditorArtefacts(): ReactElement {
  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);

  return (
    <SpawnChunkView<SpawnArtefactSpawnsChunk>
      chunk={spawnFileService.artefactSpawn}
      render={(chunk: SpawnArtefactSpawnsChunk) => <SpawnEditorArtefactsNodesTable nodes={chunk.nodes} />}
      onLoad={spawnFileService.loadArtefactSpawn}
    />
  );
}
