import { Box, CircularProgress, Divider, Grid, Typography } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { SpawnEditorHeaderTable } from "@/applications/spawn_editor/components/editor/chunks/header/SpawnEditorHeaderTable";
import { SpawnFileService } from "@/applications/spawn_editor/store/spawn";

export function SpawnEditorHeader(): ReactElement {
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
      }}
    >
      <Typography variant={"h5"}>Header</Typography>
      <Divider sx={{ margin: "16px 0" }} />
      <SpawnEditorHeaderTable header={spawnFileService.spawnFile.value.header} />
    </Box>
  );
}
