import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { SpawnEditorHeaderTable } from "@/applications/spawn/components/editor/chunks/header/SpawnEditorHeaderTable";
import { SpawnChunkView } from "@/applications/spawn/components/editor/chunks/SpawnChunkView";
import { SpawnHeaderChunk } from "@/lib/rust-sdk/xray-db";
import { SpawnFileService } from "@/lib/spawn-file";

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
