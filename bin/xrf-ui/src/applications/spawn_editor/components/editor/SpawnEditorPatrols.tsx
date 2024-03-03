import { CircularProgress, Divider, Grid, Typography } from "@mui/material";
import { DataGrid } from "@mui/x-data-grid";
import { ReactElement, useEffect } from "react";

import { TableToolbar } from "@/applications/spawn_editor/components/editor/table/TableToolbar";
import { ISpawnFilePatrols } from "@/applications/spawn_editor/types";
import { useInvokeCommand } from "@/lib/async/useInvokeCommand";
import { ECommand } from "@/lib/ipc";
import { Logger, useLogger } from "@/lib/logging";

export function SpawnEditorPatrols(): ReactElement {
  const log: Logger = useLogger("editor-patrols");

  const { isLoading, value, get, error } = useInvokeCommand<ISpawnFilePatrols>(ECommand.GET_SPAWN_FILE_PATROLS);

  useEffect(() => {
    get().then(() => log.info("Fetched patrols info"));
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

      <DataGrid
        slots={{ toolbar: TableToolbar }}
        rowHeight={24}
        rows={value.patrols.map((it) => ({
          id: it.name,
          name: it.name,
          pointsCount: it.points.length,
          linksCount: it.links.length,
        }))}
        columns={[
          { field: "name", headerName: "name", width: 300 },
          { field: "pointsCount", headerName: "points count" },
          { field: "linksCount", headerName: "links count" },
        ]}
        slotProps={{
          toolbar: {
            showQuickFilter: true,
          },
        }}
        initialState={{
          pagination: { paginationModel: { pageSize: 50 } },
        }}
      />
    </Grid>
  );
}
