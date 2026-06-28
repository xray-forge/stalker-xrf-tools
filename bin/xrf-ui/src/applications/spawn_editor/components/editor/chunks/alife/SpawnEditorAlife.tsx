import { Box, CircularProgress, Divider, Grid, Typography } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { SpawnEditorAlifeObjectsTable } from "@/applications/spawn_editor/components/editor/chunks/alife/SpawnEditorAlifeObjectsTable";
import { SpawnFileService } from "@/applications/spawn_editor/store/spawn";

export function SpawnEditorAlife(): ReactElement {
  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);

  if (spawnFileService.spawnFile.isLoading) {
    return (
      <Grid
        container
        sx={{ justifyContent: "center", alignItems: "center", width: "auto", height: "100%", flexGrow: 1 }}
      >
        <CircularProgress />
      </Grid>
    );
  }

  if (spawnFileService.spawnFile.error || !spawnFileService.spawnFile.value) {
    return (
      <Grid
        container
        sx={{ justifyContent: "center", alignItems: "center", width: "auto", height: "100%", flexGrow: 1 }}
      >
        {spawnFileService.spawnFile.error ? String(spawnFileService.spawnFile.error) : "No value."}
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
      <Typography variant={"h5"}>Alife</Typography>
      <Divider sx={{ margin: "16px 0" }} />
      <SpawnEditorAlifeObjectsTable objects={spawnFileService.spawnFile.value.alifeSpawn.objects} />
    </Box>
  );
}
