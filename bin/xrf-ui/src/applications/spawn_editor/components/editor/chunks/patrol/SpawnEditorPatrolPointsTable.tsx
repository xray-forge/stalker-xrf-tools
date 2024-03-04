import { DataGrid } from "@mui/x-data-grid";
import { GridColDef } from "@mui/x-data-grid/models/colDef/gridColDef";
import { GridRowsProp } from "@mui/x-data-grid/models/gridRows";
import { ReactElement, useMemo } from "react";

import { TableToolbar } from "@/applications/spawn_editor/components/editor/table/TableToolbar";
import { IPatrol } from "@/lib/spawn_file";

interface ISpawnEditorPatrolsTableProps {
  patrols: Array<IPatrol>;
}

export function SpawnEditorPatrolPointsTable({ patrols }: ISpawnEditorPatrolsTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      { field: "patrol", headerName: "patrol", width: 300 },
      { field: "name", headerName: "name" },
      { field: "flags", headerName: "flags" },
      { field: "level_vertex_id", headerName: "level_vertex_id", width: 120 },
      { field: "game_vertex_id", headerName: "game_vertex_id", width: 120 },
      {
        field: "position",
        headerName: "position",
        width: 160,
        valueGetter: (it) => (it.row.position ? JSON.stringify(it.row.position) : null),
      },
    ],
    [patrols]
  );

  const rows: GridRowsProp = useMemo(() => {
    const points = [];

    for (const patrol of patrols) {
      for (const point of patrol.points) {
        points.push({
          ...point,
          id: patrol.name + point.name,
          patrol: patrol.name,
        });
      }
    }

    return points;
  }, [patrols]);

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
