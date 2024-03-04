import { DataGrid } from "@mui/x-data-grid";
import { GridColDef } from "@mui/x-data-grid/models/colDef/gridColDef";
import { GridRowsProp } from "@mui/x-data-grid/models/gridRows";
import { ReactElement, useMemo } from "react";

import { TableToolbar } from "@/applications/spawn_editor/components/editor/table/TableToolbar";
import { IAlifeObjectBase } from "@/lib/spawn_file";

interface ISpawnEditorAlifeObjectsTableProps {
  objects: Array<IAlifeObjectBase>;
}

export function SpawnEditorAlifeObjectsTable({ objects }: ISpawnEditorAlifeObjectsTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      { field: "index", headerName: "index" },
      { field: "script_version", headerName: "script_version" },
      { field: "version", headerName: "version" },
      { field: "clsid", headerName: "clsid" },
      { field: "name", headerName: "name", width: 200 },
      { field: "section", headerName: "section", width: 160 },
      { field: "game_type", headerName: "game_type" },
      { field: "script_game_id", headerName: "script_game_id" },
      { field: "script_rp", headerName: "script_rp" },
      { field: "respawn_time", headerName: "respawn_time" },
      { field: "script_flags", headerName: "script_flags" },
      { field: "inner_id", headerName: "id" },
      { field: "spawn_id", headerName: "spawn_id" },
      { field: "parent_id", headerName: "parent_id" },
      { field: "phantom_id", headerName: "phantom_id" },
      { field: "net_action", headerName: "net_action" },
      { field: "client_data_size", headerName: "client_data_size" },
      {
        field: "direction",
        headerName: "direction",
        valueGetter: (it) => (it.value ? JSON.stringify(it.value) : null),
      },
      {
        field: "position",
        headerName: "position",
        valueGetter: (it) => (it.value ? JSON.stringify(it.value) : null),
      },
      {
        field: "inherited",
        headerName: "inherited",
        valueGetter: (it) => (it.row.inherited ? JSON.stringify(it.row.inherited) : null),
      },
      { field: "update_data", headerName: "update_data" },
    ],
    []
  );

  const rows: GridRowsProp = useMemo(
    () =>
      objects.map((it) => ({
        ...it,
        id: it.index,
        inner_id: it.id,
      })),
    objects
  );

  return (
    <DataGrid
      slots={{ toolbar: TableToolbar }}
      rowHeight={24}
      rows={rows}
      columns={columns}
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
