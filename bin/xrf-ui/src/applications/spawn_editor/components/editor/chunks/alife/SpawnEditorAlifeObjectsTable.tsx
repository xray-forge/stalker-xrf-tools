import { DataGrid, GridColDef, GridRowsProp } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { TableToolbar } from "@/applications/spawn_editor/components/editor/table/TableToolbar";
import { AnyObject } from "@/core/types/general";
import { IAlifeObjectBase } from "@/lib/spawn_file";

interface ISpawnEditorAlifeObjectsTableProps {
  objects: Array<IAlifeObjectBase>;
}

export function SpawnEditorAlifeObjectsTable({ objects }: ISpawnEditorAlifeObjectsTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      { field: "index", headerName: "index" },
      { field: "scriptVersion", headerName: "script version" },
      { field: "version", headerName: "version" },
      { field: "clsid", headerName: "clsid" },
      { field: "type", headerName: "type", width: 200 },
      { field: "name", headerName: "name", width: 200 },
      { field: "section", headerName: "section", width: 160 },
      { field: "gameType", headerName: "game type" },
      { field: "scriptGameId", headerName: "script game id" },
      { field: "scriptRp", headerName: "script rp" },
      { field: "respawnTime", headerName: "respawn time" },
      { field: "scriptFlags", headerName: "script flags" },
      { field: "innerId", headerName: "id" },
      { field: "spawnId", headerName: "spawn id" },
      { field: "parentId", headerName: "parent id" },
      { field: "phantomId", headerName: "phantom id" },
      { field: "netAction", headerName: "net action" },
      { field: "clientDataSize", headerName: "client data size" },
      {
        field: "direction",
        headerName: "direction",
        valueGetter: (it: AnyObject) => (it.value ? JSON.stringify(it.value) : null),
      },
      {
        field: "position",
        headerName: "position",
        valueGetter: (it: AnyObject) => (it.value ? JSON.stringify(it.value) : null),
      },
      {
        field: "inherited",
        headerName: "inherited",
        valueGetter: (it: AnyObject) => (it.row.inherited ? JSON.stringify(it.row.inherited) : null),
      },
      { field: "updateData", headerName: "update data" },
    ],
    []
  );

  const rows: GridRowsProp = useMemo(
    () =>
      objects.map((it, index) => ({
        ...it,
        id: index,
        innerId: it.id,
        type: it.inherited.type,
      })),
    objects
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
