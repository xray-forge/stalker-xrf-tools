import { DataGrid } from "@mui/x-data-grid";
import { GridColDef } from "@mui/x-data-grid/models/colDef/gridColDef";
import { GridRowsProp } from "@mui/x-data-grid/models/gridRows";
import { ReactElement, useMemo } from "react";

import { ICrossTable } from "@/lib/spawn_file";

interface ISpawnEditorGraphCrossTablesTableProps {
  crossTables: Array<ICrossTable>;
}

export function SpawnEditorGraphCrossTablesTable({
  crossTables,
}: ISpawnEditorGraphCrossTablesTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      { field: "game_guid", headerName: "game_guid", width: 240 },
      { field: "level_guid", headerName: "level_guid", width: 240 },
      { field: "version", headerName: "version" },
      { field: "nodes_count", headerName: "nodes_count" },
      { field: "vertex_count", headerName: "vertex_count" },
    ],
    []
  );

  const rows: GridRowsProp = useMemo(() => crossTables.map((it) => ({ ...it, id: it.level_guid })), [crossTables]);

  return <DataGrid rowHeight={24} rows={rows} columns={columns} />;
}
