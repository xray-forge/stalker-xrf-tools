import { DataGrid } from "@mui/x-data-grid";
import { GridColDef } from "@mui/x-data-grid/models/colDef/gridColDef";
import { GridRowsProp } from "@mui/x-data-grid/models/gridRows";
import { ReactElement, useMemo } from "react";

import { IGraphPoint } from "@/lib/spawn_file";

interface ISpawnEditorGraphPointsTableProps {
  points: Array<IGraphPoint>;
}

export function SpawnEditorGraphPointsTable({ points }: ISpawnEditorGraphPointsTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      { field: "id", headerName: "id" },
      { field: "distance", headerName: "distance", width: 240 },
      { field: "level_vertex_id", headerName: "level_vertex_id", width: 160 },
    ],
    []
  );

  const rows: GridRowsProp = useMemo(() => points.map((it, index) => ({ ...it, id: index })), [points]);

  return <DataGrid rowHeight={24} rows={rows} columns={columns} />;
}
