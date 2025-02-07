import { DataGrid, GridColDef, GridRowsProp } from "@mui/x-data-grid";
import { ReactElement, useMemo } from "react";

import { TableToolbar } from "@/applications/spawn_editor/components/editor/table/TableToolbar";
import { AnyObject } from "@/core/types/general";
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
      { field: "levelVertexId", headerName: "level vertex id", width: 120 },
      { field: "gameVertexId", headerName: "game vertex id", width: 120 },
      {
        field: "position",
        headerName: "position",
        width: 160,
        valueGetter: (it: AnyObject) => (it.row.position ? JSON.stringify(it.row.position) : null),
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
