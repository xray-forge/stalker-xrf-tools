import { Box, CircularProgress, Divider, Grid, Tab, Tabs, Typography } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement, useMemo } from "react";

import { SpawnEditorPatrolLinksTable } from "@/applications/spawn_editor/components/editor/chunks/patrol/SpawnEditorPatrolLinksTable";
import { SpawnEditorPatrolPointsTable } from "@/applications/spawn_editor/components/editor/chunks/patrol/SpawnEditorPatrolPointsTable";
import { SpawnEditorPatrolsTable } from "@/applications/spawn_editor/components/editor/chunks/patrol/SpawnEditorPatrolsTable";
import { SpawnFileManager } from "@/applications/spawn_editor/store/spawn";
import { useTabState } from "@/lib/tab";

export function SpawnEditorPatrols({
  spawnContext: { spawnFile: { value: spawnFile, isLoading, error } } = useManager(SpawnFileManager),
}): ReactElement {
  const [activeTab, , onActiveTabChange] = useTabState<string>("patrols");

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

  const activeTable: ReactElement = useMemo(() => {
    switch (activeTab) {
      case "patrols":
        return <SpawnEditorPatrolsTable patrols={spawnFile.patrols.patrols} />;

      case "points":
        return <SpawnEditorPatrolPointsTable patrols={spawnFile.patrols.patrols} />;

      case "links":
        return <SpawnEditorPatrolLinksTable patrols={spawnFile.patrols.patrols} />;

      default:
        return <Grid>Unknown tab</Grid>;
    }
  }, [activeTab, spawnFile.patrols]);

  return (
    <Grid
      width={"auto"}
      height={"100%"}
      direction={"column"}
      overflow={"auto"}
      p={2}
      flexGrow={1}
      flexWrap={"nowrap"}
      container
    >
      <Typography variant={"h5"}>Patrols</Typography>
      <Divider sx={{ margin: "16px 0" }} />

      <Tabs value={activeTab} onChange={onActiveTabChange}>
        <Tab value={"patrols"} label={"Patrols"} />
        <Tab value={"points"} label={"Points"} />
        <Tab value={"links"} label={"Links"} />
      </Tabs>

      <Box marginBottom={1} />

      {activeTable}
    </Grid>
  );
}
