import { DataGrid, GridColDef, GridRowsProp } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { TableToolbar } from "@/applications/spawn_editor/components/editor/table/TableToolbar";
import { IGraphEdge } from "@/lib/spawn_file";

interface ISpawnEditorGraphEdgesTableProps {
  edges: Array<IGraphEdge>;
}

export function SpawnEditorGraphEdgesTable({ edges }: ISpawnEditorGraphEdgesTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      { field: "id", headerName: "id" },
      { field: "distance", headerName: "distance", width: 240 },
      { field: "game_vertex_id", headerName: "game_vertex_id", width: 160 },
    ],
    []
  );

  const rows: GridRowsProp = useMemo(() => edges.map((it, index) => ({ ...it, id: index })), [edges]);

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
