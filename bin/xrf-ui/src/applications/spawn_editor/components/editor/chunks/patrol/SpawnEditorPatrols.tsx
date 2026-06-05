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

  const activeTable: ReactElement = useMemo(() => {
    if (!spawnFile) {
      return <Box>No file</Box>;
    }

    switch (activeTab) {
      case "patrols":
        return <SpawnEditorPatrolsTable patrols={spawnFile.patrols.patrols} />;

      case "points":
        return <SpawnEditorPatrolPointsTable patrols={spawnFile.patrols.patrols} />;

      case "links":
        return <SpawnEditorPatrolLinksTable patrols={spawnFile.patrols.patrols} />;

      default:
        return <Box>Unknown tab</Box>;
    }
  }, [activeTab, spawnFile?.patrols]);

  if (isLoading) {
    return (
      <Grid
        container
        sx={{ justifyContent: "center", alignItems: "center", width: "auto", height: "100%", flexGrow: 1 }}
      >
        <CircularProgress />
      </Grid>
    );
  }

  if (error || !spawnFile) {
    return (
      <Grid
        container
        sx={{ justifyContent: "center", alignItems: "center", width: "auto", height: "100%", flexGrow: 1 }}
      >
        {error ? String(error) : "No value."}
      </Grid>
    );
  }

  return (
    <Box
      sx={{
        display: "flex",
        width: "auto",
        height: "100%",
        flexDirection: "column",
        overflow: "auto",
        p: 2,
        flexGrow: 1,
        flexWrap: "nowrap",
      }}
    >
      <Typography variant={"h5"}>Patrols</Typography>
      <Divider sx={{ margin: "16px 0" }} />

      <Tabs value={activeTab} onChange={onActiveTabChange}>
        <Tab value={"patrols"} label={"Patrols"} />
        <Tab value={"points"} label={"Points"} />
        <Tab value={"links"} label={"Links"} />
      </Tabs>

      <Box sx={{ marginBottom: 1 }} />

      {activeTable}
    </Box>
  );
}
