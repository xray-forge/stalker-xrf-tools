import { Box, CircularProgress, Divider, Grid, Typography } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { SpawnEditorAlifeObjectsTable } from "@/applications/spawn_editor/components/editor/chunks/alife/SpawnEditorAlifeObjectsTable";
import { SpawnFileService } from "@/applications/spawn_editor/store/spawn";

export function SpawnEditorAlife(): ReactElement {
  const {
    spawnFile: { value: spawnFile, isLoading, error },
  } = useInjection(SpawnFileService);

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
      <Typography variant={"h5"}>Alife</Typography>
      <Divider sx={{ margin: "16px 0" }} />
      <SpawnEditorAlifeObjectsTable objects={spawnFile.alifeSpawn.objects} />
    </Box>
  );
}
