import { Box, CircularProgress, Divider, Grid, Tab, Tabs, Typography } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement, useMemo } from "react";

import { SpawnEditorGraphCrossTable } from "@/applications/spawn_editor/components/editor/chunks/graph/SpawnEditorGraphCrossTable";
import { SpawnEditorGraphEdgesTable } from "@/applications/spawn_editor/components/editor/chunks/graph/SpawnEditorGraphEdgesTable";
import { SpawnEditorGraphHeaderTable } from "@/applications/spawn_editor/components/editor/chunks/graph/SpawnEditorGraphHeaderTable";
import { SpawnEditorGraphLevelsTable } from "@/applications/spawn_editor/components/editor/chunks/graph/SpawnEditorGraphLevelsTable";
import { SpawnEditorGraphPointsTable } from "@/applications/spawn_editor/components/editor/chunks/graph/SpawnEditorGraphPointsTable";
import { SpawnEditorGraphVerticesTable } from "@/applications/spawn_editor/components/editor/chunks/graph/SpawnEditorGraphVerticesTable";
import { SpawnFileManager } from "@/applications/spawn_editor/store/spawn";
import { useTabState } from "@/lib/tab";

export function SpawnEditorGraphs({
  spawnContext: { spawnFile: { value: spawnFile, isLoading, error } } = useManager(SpawnFileManager),
}): ReactElement {
  const [activeTab, , onActiveTabChange] = useTabState<string>("header");

  const activeTable: ReactElement = useMemo(() => {
    if (!spawnFile) {
      return <Grid>No file</Grid>;
    }

    switch (activeTab) {
      case "header":
        return <SpawnEditorGraphHeaderTable header={spawnFile.graphs.header} />;

      case "levels":
        return <SpawnEditorGraphLevelsTable levels={spawnFile.graphs.levels} />;

      case "edges":
        return <SpawnEditorGraphEdgesTable edges={spawnFile.graphs.edges} />;

      case "points":
        return <SpawnEditorGraphPointsTable points={spawnFile.graphs.points} />;

      case "vertices":
        return <SpawnEditorGraphVerticesTable vertices={spawnFile.graphs.vertices} />;

      case "cross_tables":
        return <SpawnEditorGraphCrossTable crossTables={spawnFile.graphs.cross_tables} />;

      default:
        return <Grid>Unknown tab</Grid>;
    }
  }, [activeTab, spawnFile?.graphs]);

  if (isLoading) {
    return (
      <Grid justifyContent={"center"} alignItems={"center"} width={"auto"} height={"100%"} flexGrow={1} container>
        <CircularProgress />
      </Grid>
    );
  }

  if (error || !spawnFile) {
    return (
      <Grid justifyContent={"center"} alignItems={"center"} width={"auto"} height={"100%"} flexGrow={1} container>
        {error ? String(error) : "No value."}
      </Grid>
    );
  }

  return (
    <Grid width={"auto"} height={"100%"} direction={"column"} overflow={"auto"} p={2} flexGrow={1} container>
      <Typography variant={"h5"}>Graph</Typography>

      <Divider sx={{ margin: "16px 0" }} />

      <Tabs value={activeTab} onChange={onActiveTabChange}>
        <Tab value={"header"} label={"Header"} />
        <Tab value={"levels"} label={"Levels"} />
        <Tab value={"edges"} label={"Edges"} />
        <Tab value={"points"} label={"Points"} />
        <Tab value={"vertices"} label={"Vertices"} />
        <Tab value={"cross_tables"} label={"Cross tables"} />
      </Tabs>

      <Box marginBottom={1} />

      {activeTable}
    </Grid>
  );
}
