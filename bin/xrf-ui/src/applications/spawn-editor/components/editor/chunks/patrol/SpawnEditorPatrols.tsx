import { Box, Tab, Tabs } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { SpawnEditorPatrolLinksTable } from "@/applications/spawn-editor/components/editor/chunks/patrol/SpawnEditorPatrolLinksTable";
import { SpawnEditorPatrolPointsTable } from "@/applications/spawn-editor/components/editor/chunks/patrol/SpawnEditorPatrolPointsTable";
import { SpawnEditorPatrolsTable } from "@/applications/spawn-editor/components/editor/chunks/patrol/SpawnEditorPatrolsTable";
import { SpawnChunkView } from "@/applications/spawn-editor/components/editor/chunks/SpawnChunkView";
import { TChunkTabChange, useChunkTab } from "@/applications/spawn-editor/components/editor/chunks/use-chunk-tab";
import { SpawnFileService } from "@/applications/spawn-editor/store/spawn";
import { ISpawnFilePatrolsChunk } from "@/lib/spawn-file";

const BASE_PATH: string = "/spawn-editor/editor/patrols";
const TABS: Array<string> = ["patrols", "points", "links"];

export function SpawnEditorPatrols(): ReactElement {
  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);

  const [activeTab, onChangeTab]: [string, TChunkTabChange] = useChunkTab(BASE_PATH, TABS, "patrols");

  return (
    <SpawnChunkView<ISpawnFilePatrolsChunk>
      chunk={spawnFileService.patrols}
      render={(chunk: ISpawnFilePatrolsChunk) => (
        <>
          <Tabs value={activeTab} sx={{ marginBottom: 1, flexShrink: 0 }} onChange={onChangeTab}>
            <Tab value={"patrols"} label={"Patrols"} />
            <Tab value={"points"} label={"Points"} />
            <Tab value={"links"} label={"Links"} />
          </Tabs>

          <Box sx={{ display: "flex", flexGrow: 1, minHeight: 0 }}>
            {activeTab === "points" ? <SpawnEditorPatrolPointsTable patrols={chunk.patrols} /> : null}
            {activeTab === "links" ? <SpawnEditorPatrolLinksTable patrols={chunk.patrols} /> : null}
            {activeTab === "patrols" ? <SpawnEditorPatrolsTable patrols={chunk.patrols} /> : null}
          </Box>
        </>
      )}
      onLoad={spawnFileService.loadPatrols}
    />
  );
}
