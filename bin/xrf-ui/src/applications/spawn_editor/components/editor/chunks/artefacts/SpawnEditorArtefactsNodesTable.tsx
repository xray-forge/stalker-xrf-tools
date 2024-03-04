import { DataGrid, GridColDef, GridRowsProp } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { TableToolbar } from "@/applications/spawn_editor/components/editor/table/TableToolbar";
import { IArtefactSpawnNode } from "@/lib/spawn_file";

interface ISpawnEditorArtefactsNodesTableProps {
  nodes: Array<IArtefactSpawnNode>;
}

export function SpawnEditorArtefactsNodesTable({ nodes }: ISpawnEditorArtefactsNodesTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      { field: "id", headerName: "id" },
      { field: "levelVertexId", headerName: "level vertex id", width: 120 },
      { field: "distance", headerName: "distance", width: 172 },
      {
        field: "position",
        headerName: "position",
        width: 240,
        valueGetter: (it) => (it.value ? JSON.stringify(it.value) : null),
      },
    ],
    []
  );

  const rows: GridRowsProp = useMemo(
    () =>
      nodes.map((it, index) => ({
        ...it,
        id: index,
      })),
    [nodes]
  );

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
