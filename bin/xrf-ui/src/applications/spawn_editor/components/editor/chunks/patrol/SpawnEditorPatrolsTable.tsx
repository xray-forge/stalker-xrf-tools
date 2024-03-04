import { DataGrid } from "@mui/x-data-grid";
import { GridColDef } from "@mui/x-data-grid/models/colDef/gridColDef";
import { GridRowsProp } from "@mui/x-data-grid/models/gridRows";
import { ReactElement, useMemo } from "react";

import { TableToolbar } from "@/applications/spawn_editor/components/editor/table/TableToolbar";
import { IPatrol } from "@/lib/spawn_file";

interface ISpawnEditorPatrolsTableProps {
  patrols: Array<IPatrol>;
}

export function SpawnEditorPatrolsTable({ patrols }: ISpawnEditorPatrolsTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      { field: "name", headerName: "patrol", width: 300 },
      { field: "pointsCount", headerName: "points count" },
      { field: "linksCount", headerName: "links count" },
    ],
    [patrols]
  );

  const rows: GridRowsProp = useMemo(
    () =>
      patrols.map((it) => ({
        id: it.name,
        name: it.name,
        pointsCount: it.points.length,
        linksCount: it.links.length,
      })),
    [patrols]
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
