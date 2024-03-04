import { DataGrid, GridColDef, GridRowsProp } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { TableToolbar } from "@/applications/spawn_editor/components/editor/table/TableToolbar";
import { IGraphVertex } from "@/lib/spawn_file";

interface ISpawnEditorGraphVerticesTableProps {
  vertices: Array<IGraphVertex>;
}

export function SpawnEditorGraphVerticesTable({ vertices }: ISpawnEditorGraphVerticesTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      { field: "id", headerName: "id" },
      { field: "edge_count", headerName: "edge_count" },
      { field: "edge_offset", headerName: "edge_offset" },
      { field: "level_id", headerName: "level_id" },
      { field: "level_point_count", headerName: "level_point_count", width: 132 },
      { field: "level_point_offset", headerName: "level_point_offset", width: 132 },
      { field: "level_vertex_id", headerName: "level_vertex_id", width: 132 },
      { field: "vertex_type", headerName: "vertex_type" },
      {
        field: "game_point",
        headerName: "game_point",
        valueGetter: (it) => (it.value ? JSON.stringify(it.value) : null),
      },
      {
        field: "level_point",
        headerName: "level_point",
        valueGetter: (it) => (it.value ? JSON.stringify(it.value) : null),
      },
    ],
    []
  );

  const rows: GridRowsProp = useMemo(() => vertices.map((it, index) => ({ ...it, id: index })), [vertices]);

  return (
    <DataGrid
      rowHeight={24}
      rows={rows}
      columns={columns}
      sx={{
        maxWidth: "100%",
      }}
      slots={{ toolbar: TableToolbar }}
      slotProps={{
        toolbar: {
          showQuickFilter: true,
        },
      }}
      initialState={{
        pagination: { paginationModel: { pageSize: 50 } },
      }}
    />
  );
}
