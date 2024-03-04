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
      { field: "edgesCount", headerName: "edges count" },
      { field: "edgesOffset", headerName: "edges offset" },
      { field: "levelId", headerName: "level id" },
      { field: "levelPointsCount", headerName: "level points count", width: 132 },
      { field: "levelPointsOffset", headerName: "level points offset", width: 132 },
      { field: "levelVertexId", headerName: "level vertex id", width: 132 },
      { field: "vertexType", headerName: "vertex type" },
      {
        field: "gamePoint",
        headerName: "game point",
        valueGetter: (it) => (it.value ? JSON.stringify(it.value) : null),
      },
      {
        field: "levelPoint",
        headerName: "level point",
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
