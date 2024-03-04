import { CircularProgress, Divider, Grid, Typography } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement } from "react";

import { SpawnEditorArtefactsNodesTable } from "@/applications/spawn_editor/components/editor/chunks/artefacts/SpawnEditorArtefactsNodesTable";
import { SpawnFileManager } from "@/applications/spawn_editor/store/spawn";

export function SpawnEditorArtefacts({
  spawnContext: { spawnFile: { value: spawnFile, isLoading, error } } = useManager(SpawnFileManager),
}): ReactElement {
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
      <Typography variant={"h5"}>Artefact spawn nodes</Typography>
      <Divider sx={{ margin: "16px 0" }} />
      <SpawnEditorArtefactsNodesTable nodes={spawnFile.artefact_spawn.nodes} />
    </Grid>
  );
}
