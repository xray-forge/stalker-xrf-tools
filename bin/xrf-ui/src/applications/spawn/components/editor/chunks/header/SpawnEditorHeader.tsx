import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { SpawnEditorHeaderTable } from "@/applications/spawn/components/editor/chunks/header/SpawnEditorHeaderTable";
import { SpawnChunkView } from "@/applications/spawn/components/editor/chunks/SpawnChunkView";
import { ISpawnFileHeaderChunk, SpawnFileService } from "@/lib/spawn-file";

export function SpawnEditorHeader(): ReactElement {
  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);

  return (
    <SpawnChunkView<ISpawnFileHeaderChunk>
      chunk={spawnFileService.header}
      render={(header: ISpawnFileHeaderChunk) => <SpawnEditorHeaderTable header={header} />}
      onLoad={spawnFileService.loadHeader}
    />
  );
}
