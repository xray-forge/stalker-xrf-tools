import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { SpawnEditorHeaderTable } from "@/applications/spawn/components/editor/chunks/header/SpawnEditorHeaderTable";
import { SpawnChunkView } from "@/applications/spawn/components/editor/chunks/SpawnChunkView";
import { SpawnHeaderChunk } from "@/lib/xrf/bindings/xrf-db";
import { SpawnFileService } from "@/lib/xrf/spawn-file";

export function SpawnEditorHeader(): ReactElement {
  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);

  return (
    <SpawnChunkView<SpawnHeaderChunk>
      chunk={spawnFileService.header}
      render={(header: SpawnHeaderChunk) => <SpawnEditorHeaderTable header={header} />}
      onLoad={spawnFileService.loadHeader}
    />
  );
}
