import { DataGrid } from "@mui/x-data-grid";
import { GridColDef } from "@mui/x-data-grid/models/colDef/gridColDef";
import { GridRowsProp } from "@mui/x-data-grid/models/gridRows";
import { ReactElement, useMemo } from "react";

import { TableToolbar } from "@/applications/spawn_editor/components/editor/table/TableToolbar";
import { IPatrol } from "@/lib/spawn_file";

interface ISpawnEditorPatrolsTableProps {
  patrols: Array<IPatrol>;
}

export function SpawnEditorPatrolLinksTable({ patrols }: ISpawnEditorPatrolsTableProps): ReactElement {
  const columns: Array<GridColDef> = useMemo(
    () => [
      { field: "patrol", headerName: "patrol", width: 300 },
      { field: "index", headerName: "index" },
      {
        field: "links",
        headerName: "links",
        width: 160,
        valueGetter: (it) => (it.row.links ? JSON.stringify(it.row.links) : null),
      },
    ],
    [patrols]
  );

  const rows: GridRowsProp = useMemo(() => {
    const links = [];

    for (const patrol of patrols) {
      for (const link of patrol.links) {
        links.push({
          ...link,
          id: patrol.name + link.index,
          patrol: patrol.name,
        });
      }
    }

    return links;
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
