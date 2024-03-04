import { DataGrid } from "@mui/x-data-grid";
import { GridColDef } from "@mui/x-data-grid/models/colDef/gridColDef";
import { GridRowsProp } from "@mui/x-data-grid/models/gridRows";
import { ReactElement, useMemo } from "react";

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

  return <DataGrid rowHeight={24} rows={rows} columns={columns} />;
}
