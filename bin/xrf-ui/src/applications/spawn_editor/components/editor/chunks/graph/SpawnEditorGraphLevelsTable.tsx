import { DataGrid } from "@mui/x-data-grid";
import { GridColDef } from "@mui/x-data-grid/models/colDef/gridColDef";
import { GridRowsProp } from "@mui/x-data-grid/models/gridRows";
import { ReactElement, useMemo } from "react";

import { IGraphLevel } from "@/lib/spawn_file";

interface ISpawnEditorGraphLevelsTableProps {
  levels: Array<IGraphLevel>;
}

export function SpawnEditorGraphLevelsTable({ levels }: ISpawnEditorGraphLevelsTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      { field: "id", headerName: "id" },
      { field: "name", headerName: "name", width: 160 },
      { field: "section", headerName: "section" },
      { field: "guid", headerName: "guid", width: 240 },
      {
        field: "offset",
        headerName: "offset",
        valueGetter: (it) => (it.value ? JSON.stringify(it.value) : null),
      },
    ],
    []
  );

  const rows: GridRowsProp = useMemo(() => levels, [levels]);

  return <DataGrid rowHeight={24} rows={rows} columns={columns} />;
}
