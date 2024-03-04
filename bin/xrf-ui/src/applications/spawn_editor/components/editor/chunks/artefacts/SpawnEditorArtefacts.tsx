import { CircularProgress, Divider, Grid, Typography } from "@mui/material";
import { DataGrid } from "@mui/x-data-grid";
import { GridColDef } from "@mui/x-data-grid/models/colDef/gridColDef";
import { GridRowsProp } from "@mui/x-data-grid/models/gridRows";
import { useManager } from "dreamstate";
import { ReactElement } from "react";

import { TableToolbar } from "@/applications/spawn_editor/components/editor/table/TableToolbar";
import { SpawnFileManager } from "@/applications/spawn_editor/store/spawn";

export function SpawnEditorArtefacts({ spawnContext: { spawnFile } = useManager(SpawnFileManager) }): ReactElement {
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
    { field: "index", headerName: "index" },
    { field: "level_vertex_id", headerName: "level_vertex_id", width: 120 },
    { field: "distance", headerName: "distance", width: 172 },
    {
      field: "position",
      headerName: "position",
      width: 240,
      valueGetter: (it) => (it.value ? JSON.stringify(it.value) : null),
    },
  ];

  const rows: GridRowsProp = spawnFile.value.artefact_spawn.nodes.map((it, index) => ({
    id: index,
    index: index,
    position: it.position,
    distance: it.distance,
    level_vertex_id: it.level_vertex_id,
  }));

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
      <DataGrid
        slots={{ toolbar: TableToolbar }}
        rowHeight={24}
        rows={rows}
        columns={columns}
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
