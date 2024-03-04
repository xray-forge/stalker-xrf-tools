import { DataGrid, GridColDef, GridRowsProp } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { TableToolbar } from "@/applications/spawn_editor/components/editor/table/TableToolbar";
import { ICrossTable } from "@/lib/spawn_file";

interface ISpawnEditorGraphCrossTableProps {
  crossTables: Array<ICrossTable>;
}

export function SpawnEditorGraphCrossTable({ crossTables }: ISpawnEditorGraphCrossTableProps): ReactElement {
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
