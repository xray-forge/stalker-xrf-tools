import { DataGrid } from "@mui/x-data-grid";
import { GridColDef } from "@mui/x-data-grid/models/colDef/gridColDef";
import { GridRowsProp } from "@mui/x-data-grid/models/gridRows";
import { ReactElement, useMemo } from "react";

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

  return <DataGrid rowHeight={24} rows={rows} columns={columns} />;
}
