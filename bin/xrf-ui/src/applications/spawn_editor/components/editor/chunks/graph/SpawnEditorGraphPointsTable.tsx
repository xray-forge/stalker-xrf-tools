import { DataGrid, GridColDef, GridRowsProp } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { TableToolbar } from "@/applications/spawn_editor/components/editor/table/TableToolbar";
import { IGraphPoint } from "@/lib/spawn_file";

interface ISpawnEditorGraphPointsTableProps {
  points: Array<IGraphPoint>;
}

export function SpawnEditorGraphPointsTable({ points }: ISpawnEditorGraphPointsTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      { field: "id", headerName: "id" },
      { field: "distance", headerName: "distance", width: 240 },
      { field: "levelVertexId", headerName: "level vertex id", width: 160 },
    ],
    []
  );

  const rows: GridRowsProp = useMemo(() => points.map((it, index) => ({ ...it, id: index })), [points]);

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
