import { Box, Tab, Tabs } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { SpawnEditorGraphCrossTable } from "@/applications/spawn-editor/components/editor/chunks/graph/SpawnEditorGraphCrossTable";
import { SpawnEditorGraphEdgesTable } from "@/applications/spawn-editor/components/editor/chunks/graph/SpawnEditorGraphEdgesTable";
import { SpawnEditorGraphHeaderTable } from "@/applications/spawn-editor/components/editor/chunks/graph/SpawnEditorGraphHeaderTable";
import { SpawnEditorGraphLevelsTable } from "@/applications/spawn-editor/components/editor/chunks/graph/SpawnEditorGraphLevelsTable";
import { SpawnEditorGraphPointsTable } from "@/applications/spawn-editor/components/editor/chunks/graph/SpawnEditorGraphPointsTable";
import { SpawnEditorGraphVerticesTable } from "@/applications/spawn-editor/components/editor/chunks/graph/SpawnEditorGraphVerticesTable";
import { SpawnChunkView } from "@/applications/spawn-editor/components/editor/chunks/SpawnChunkView";
import { TChunkTabChange, useChunkTab } from "@/applications/spawn-editor/components/editor/chunks/use-chunk-tab";
import { SpawnFileService } from "@/applications/spawn-editor/store/spawn";
import { ISpawnFileGraphsChunk } from "@/lib/spawn-file";

const BASE_PATH: string = "/spawn-editor/editor/graph";
const TABS: Array<string> = ["header", "levels", "vertices", "edges", "points", "cross-tables"];

export function SpawnEditorGraphs(): ReactElement {
  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);

  const [activeTab, onChangeTab]: [string, TChunkTabChange] = useChunkTab(BASE_PATH, TABS, "header");

  return (
    <SpawnChunkView<ISpawnFileGraphsChunk>
      chunk={spawnFileService.graphs}
      render={(chunk: ISpawnFileGraphsChunk) => (
        <>
          <Tabs value={activeTab} variant={"scrollable"} sx={{ marginBottom: 1, flexShrink: 0 }} onChange={onChangeTab}>
            <Tab value={"header"} label={"Header"} />
            <Tab value={"levels"} label={"Levels"} />
            <Tab value={"vertices"} label={"Vertices"} />
            <Tab value={"edges"} label={"Edges"} />
            <Tab value={"points"} label={"Points"} />
            <Tab value={"cross-tables"} label={"Cross tables"} />
          </Tabs>

          <Box sx={{ display: "flex", flexGrow: 1, minHeight: 0 }}>
            {activeTab === "levels" ? <SpawnEditorGraphLevelsTable levels={chunk.levels} /> : null}
            {activeTab === "vertices" ? <SpawnEditorGraphVerticesTable vertices={chunk.vertices} /> : null}
            {activeTab === "edges" ? <SpawnEditorGraphEdgesTable edges={chunk.edges} /> : null}
            {activeTab === "points" ? <SpawnEditorGraphPointsTable points={chunk.points} /> : null}
            {activeTab === "cross-tables" ? <SpawnEditorGraphCrossTable crossTables={chunk.crossTables} /> : null}
            {activeTab === "header" ? <SpawnEditorGraphHeaderTable header={chunk.header} /> : null}
          </Box>
        </>
      )}
      onLoad={spawnFileService.loadGraphs}
    />
  );
}
