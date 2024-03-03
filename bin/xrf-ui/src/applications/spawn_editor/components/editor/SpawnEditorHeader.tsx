import { CircularProgress, Divider, Grid, Typography } from "@mui/material";
import { DataGrid } from "@mui/x-data-grid";
import { ReactElement, useEffect } from "react";

import { ISpawnFileHeader } from "@/applications/spawn_editor/types";
import { useInvokeCommand } from "@/lib/async/useInvokeCommand";
import { ECommand } from "@/lib/ipc";
import { Logger, useLogger } from "@/lib/logging";

export function SpawnEditorHeader(): ReactElement {
  const log: Logger = useLogger("editor-header");

  const { isLoading, value, get, error } = useInvokeCommand<ISpawnFileHeader>(ECommand.GET_SPAWN_FILE_HEADER);

  useEffect(() => {
    get().then(() => log.info("Fetched header info"));
  }, []);

  if (isLoading) {
    return (
      <Grid justifyContent={"center"} alignItems={"center"} width={"auto"} height={"100%"} flexGrow={1} container>
        <CircularProgress />
      </Grid>
    );
  }

  if (error || !value) {
    return (
      <Grid justifyContent={"center"} alignItems={"center"} width={"auto"} height={"100%"} flexGrow={1} container>
        {error || "No value."}
      </Grid>
    );
  }

  return (
    <Grid width={"auto"} height={"100%"} direction={"column"} overflow={"auto"} p={2} flexGrow={1} container>
      <Typography variant={"h5"}>Header</Typography>

      <Divider sx={{ margin: "16px 0" }} />

      <DataGrid
        rowHeight={24}
        rows={[{ ...value, id: "main" }]}
        columns={[
          { field: "version", headerName: "version" },
          { field: "objects_count", headerName: "objects_count" },
          { field: "level_count", headerName: "level_count" },
          { field: "guid", headerName: "guid" },
          { field: "graph_guid", headerName: "graph_guid" },
        ]}
      />
    </Grid>
  );
}
