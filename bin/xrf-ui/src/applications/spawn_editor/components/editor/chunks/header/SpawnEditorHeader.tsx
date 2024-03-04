import { CircularProgress, Divider, Grid, Typography } from "@mui/material";
import { DataGrid } from "@mui/x-data-grid";
import { GridColDef } from "@mui/x-data-grid/models/colDef/gridColDef";
import { GridRowsProp } from "@mui/x-data-grid/models/gridRows";
import { useManager } from "dreamstate";
import { ReactElement } from "react";

import { SpawnFileManager } from "@/applications/spawn_editor/store/spawn";

export function SpawnEditorHeader({ spawnContext: { spawnFile } = useManager(SpawnFileManager) }): ReactElement {
  if (spawnFile.isLoading) {
    return (
      <Grid justifyContent={"center"} alignItems={"center"} width={"auto"} height={"100%"} flexGrow={1} container>
        <CircularProgress />
      </Grid>
    );
  }

  if (spawnFile.error || !spawnFile.value) {
    return (
      <Grid justifyContent={"center"} alignItems={"center"} width={"auto"} height={"100%"} flexGrow={1} container>
        {spawnFile.error ? String(spawnFile.error) : "No value."}
      </Grid>
    );
  }

  const columns: Array<GridColDef> = [
    { field: "version", headerName: "version" },
    { field: "objects_count", headerName: "objects_count" },
    { field: "level_count", headerName: "level_count" },
    { field: "guid", headerName: "guid" },
    { field: "graph_guid", headerName: "graph_guid" },
  ];

  const rows: GridRowsProp = [{ ...spawnFile.value?.header, id: "main" }];

  return (
    <Grid width={"auto"} height={"100%"} direction={"column"} overflow={"auto"} p={2} flexGrow={1} container>
      <Typography variant={"h5"}>Header</Typography>
      <Divider sx={{ margin: "16px 0" }} />
      <DataGrid rowHeight={24} rows={rows} columns={columns} />
    </Grid>
  );
}
